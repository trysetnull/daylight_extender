use anyhow::Result;
use chrono::{Datelike, Local, LocalResult, TimeZone, Timelike};
use log::{trace, warn};
use shelly::api::Gen2DeviceClient;
use shelly::data::{ScheduleJobMethod, ScheduleJobWithOptionalId, KEY_NOT_FOUND};
use shelly::error::ShellyRpcError;

pub mod error;
use crate::error::CustomError;

pub const SCHEDULE_JOB_ID: &str = "daylight.extender.job.id";
const THIRTY_MINS_AS_SEC: i64 = 30 * 60;

#[derive(Debug)]
pub struct Controller<'a> {
    client: &'a Gen2DeviceClient<'a>,
}

impl<'a> Controller<'a> {
    pub fn new(client: &'a Gen2DeviceClient<'a>) -> Self {
        Self { client }
    }

    pub async fn execute(&self, day_length_hours: u8) -> Result<u32> {
        let day_length_seconds = i64::from(day_length_hours) * 60 * 60;

        let (sunrise, sunset) = self.get_sunrise_sunset().await?;
        let (light_on, toggle_after) =
            Self::light_on_toggle_after(sunrise, sunset, day_length_seconds)?;
        self.create_or_update_schedule(light_on, toggle_after).await
    }

    async fn get_sunrise_sunset(&self) -> Result<(i64, i64)> {
        trace!("get_sunrise_sunset");
        let (latitude, longitude) = self.client.get_location().await?;
        let timestamp = self.client.get_time().await?;

        if let LocalResult::Single(dt) = Local.timestamp_opt(timestamp, 0) {
            let (sunrise, sunset) =
                sunrise::sunrise_sunset(latitude, longitude, dt.year(), dt.month(), dt.day());
            Ok((sunrise, sunset))
        } else {
            Err(CustomError::ChronoError("timestamp out of range").into())
        }
    }

    async fn create_or_update_schedule(&self, light_on: i64, toggle_after: i64) -> Result<u32> {
        let switch_id = 0;
        let enable = light_on > 0;

        match self.client.get_value(SCHEDULE_JOB_ID).await {
            Ok(job_id_str) => {
                // Update
                let job_id = job_id_str.parse().expect("Not a valid u32");

                // if enable is false, turn the job off or do nothing.
                if !enable {
                    let result = self.client.list_schedule().await?;
                    for job in result.result.jobs {
                        if job.id.is_none() {
                            continue;
                        };

                        if job.id.unwrap() == job_id {
                            if enable == job.enable {
                                return Ok(result.result.rev);
                            }

                            let result = self.client.disable_schedule(job_id).await?;
                            return Ok(result.result.rev);
                        }
                    }
                }

                let update = Self::new_schedule_job_for_update(
                    light_on,
                    switch_id,
                    toggle_after,
                    job_id,
                    enable,
                )?;
                let result = self.client.update_schedule(&update).await?;
                Ok(result.result.rev)
            }
            Err(ShellyRpcError::HttpApiError(e)) => {
                // Create
                if e.error.code != KEY_NOT_FOUND {
                    warn!(
                        "Unexpected error received, refusing to create Schedule job: {:?}",
                        e
                    );
                    return Err(ShellyRpcError::HttpApiError(e).into());
                }

                let create =
                    Self::new_schedule_job_for_create(light_on, switch_id, toggle_after, enable)?;
                let result = self.client.create_schedule(&create).await?;
                let value = result.result.id.to_string();
                self.client
                    .set_value(SCHEDULE_JOB_ID, value.as_str())
                    .await?;

                Ok(result.result.rev)
            }
            Err(e) => Err(e.into()),
        }
    }

    fn new_schedule_job_for_update(
        light_on: i64,
        switch_id: u8,
        toggle_after: i64,
        job_id: u32,
        enable: bool,
    ) -> Result<ScheduleJobWithOptionalId, anyhow::Error> {
        Self::new_schedule_job(light_on, switch_id, toggle_after, Some(job_id), enable)
    }

    fn new_schedule_job_for_create(
        light_on: i64,
        switch_id: u8,
        toggle_after: i64,
        enable: bool,
    ) -> Result<ScheduleJobWithOptionalId, anyhow::Error> {
        Self::new_schedule_job(light_on, switch_id, toggle_after, None, enable)
    }

    fn new_schedule_job(
        light_on: i64,
        switch_id: u8,
        toggle_after: i64,
        job_id: Option<u32>,
        enable: bool,
    ) -> Result<ScheduleJobWithOptionalId, anyhow::Error> {
        let timespec = Self::get_timespec(light_on)?;
        let calls = vec![Self::call_switch_on(switch_id, toggle_after)];
        let update = ScheduleJobWithOptionalId {
            id: job_id,
            enable,
            timespec,
            calls,
        };
        Ok(update)
    }

    fn call_switch_on(id: u8, toggle_after: i64) -> ScheduleJobMethod {
        ScheduleJobMethod {
            method: "switch.set".into(),
            params: Some(serde_json::json!({"on": true, "toggle_after": toggle_after, "id": id})),
        }
    }

    fn light_on_toggle_after(sunrise: i64, sunset: i64, day_length: i64) -> Result<(i64, i64)> {
        if sunrise >= sunset {
            return Err(CustomError::ChronoError("It's the end of the world").into());
        }

        if day_length < 0 {
            return Err(CustomError::ChronoError("day_length is negative").into());
        }

        let light_on = sunset - day_length;
        let toggle_after = sunrise - light_on;
        if toggle_after < THIRTY_MINS_AS_SEC {
            return Ok((-1, toggle_after));
        }

        Ok((light_on, toggle_after))
    }

    fn get_timespec(timestamp: i64) -> Result<String> {
        if let LocalResult::Single(dt) = Local.timestamp_opt(timestamp, 0) {
            Ok(format!(
                "{} {} {} * * 0,1,2,3,4,5,6",
                dt.second(),
                dt.minute(),
                dt.hour()
            ))
        } else {
            Err(CustomError::ChronoError("timestamp out of range").into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1703114939, "59 28 0 * *")]
    #[case(946684800, "0 0 1 * *")]
    #[case(1280534460, "0 1 2 * *")]
    #[case(2235254401, "1 0 1 * *")]
    #[case(2235250801, "1 0 0 * *")]
    #[case(33481897199, "59 59 23 * *")]
    fn get_timespec_parametrized(#[case] timestamp: i64, #[case] expected: &str) {
        // act
        let result = Controller::get_timespec(timestamp);

        // assert
        assert!(result.is_ok(), "Expected Ok is Error");
        assert!(result.expect("Unexpected").starts_with(expected));
    }

    #[rstest]
    #[case(1701413700, 1701442500, 12*60*60, 1701399300, 14400)]
    #[case(1907894520, 1907955120, 12*60*60, -1, -1)]
    #[case(1678512660, 1678554180, 12*60*60, -1, -1)]
    #[case(1696309800, 1696351199, 12*60*60, 1696307999, 1801)]
    fn light_on_toggle_after_parametrized(
        #[case] sunrise: i64,
        #[case] sunset: i64,
        #[case] day_length: i64,
        #[case] expected_light_on: i64,
        #[case] expected_toggle_after: i64,
    ) {
        // act
        let result = Controller::light_on_toggle_after(sunrise, sunset, day_length);

        // assert
        let (actual_light_on, actual_toggle_after) = result.expect("Unexpected");
        if expected_light_on < 0 {
            assert_eq!(expected_light_on, actual_light_on);
        } else {
            assert_eq!(expected_light_on, actual_light_on);
            assert_eq!(expected_toggle_after, actual_toggle_after);
        }
    }
}
