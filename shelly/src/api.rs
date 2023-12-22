use crate::data::{
    KeyValueStoreGetResponse, KeyValueStoreMethod, KeyValueStoreSetResponse,
    ScheduleCreateResponse, ScheduleJobWithOptionalId, ScheduleListResponse, ScheduleMethod,
    ScheduleUpdateResponse, SysGetConfigResponse, SysGetStatusResponse, SysMethod,
};
use crate::error::ShellyRpcError;
use chrono::{NaiveTime, Utc};
use log::trace;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug)]
pub struct Gen2DeviceClient<'a> {
    address: &'a str,
}

impl<'a> Gen2DeviceClient<'a> {
    pub fn new(address: &'a str) -> Self {
        Self { address }
    }

    /// https://shelly-api-docs.shelly.cloud/gen2/ComponentsAndServices/Schedule#schedulecreate
    pub async fn create_schedule(
        &self,
        job: &ScheduleJobWithOptionalId,
    ) -> Result<ScheduleCreateResponse, ShellyRpcError> {
        trace!("create_schedule");
        Self::execute_rpc(
            self.address,
            &serde_json::json!({"id": 1, "method": ScheduleMethod::Create, "params": job}),
        )
        .await
    }

    /// https://shelly-api-docs.shelly.cloud/gen2/ComponentsAndServices/Schedule#scheduleupdate
    pub async fn update_schedule(
        &self,
        job: &ScheduleJobWithOptionalId,
    ) -> Result<ScheduleUpdateResponse, ShellyRpcError> {
        trace!("update_schedule");
        Self::execute_rpc(
            self.address,
            &serde_json::json!({"id": 1, "method": ScheduleMethod::Update, "params": job}),
        )
        .await
    }

    pub async fn disable_schedule(
        &self,
        job_id: u32,
    ) -> Result<ScheduleUpdateResponse, ShellyRpcError> {
        trace!("disable_schedule");
        Self::execute_rpc(
            self.address,
            &serde_json::json!({"id": 1, "method": ScheduleMethod::Update, "params": { "id": job_id, "enable": false } }),
        )
        .await
    }

    /// https://shelly-api-docs.shelly.cloud/gen2/ComponentsAndServices/Schedule#schedulelist
    pub async fn list_schedule(&self) -> Result<ScheduleListResponse, ShellyRpcError> {
        trace!("list_schedule");
        Self::execute_rpc(
            self.address,
            &serde_json::json!({"id": 1, "method": ScheduleMethod::List}),
        )
        .await
    }

    /// Returns the get time of this [`Gen2DeviceClient`].
    /// Calls the Sys.GetStatus endpoint to retrieve the time.
    /// See: https://shelly-api-docs.shelly.cloud/gen2/ComponentsAndServices/Sys#sysgetstatus
    pub async fn get_time(&self) -> Result<i64, ShellyRpcError> {
        trace!("get_time");
        let resp: SysGetStatusResponse = Self::execute_rpc(
            self.address,
            &serde_json::json!({"id": 1, "method": SysMethod::GetStatus}),
        )
        .await?;

        if let Some(unix_timestamp) = resp.result.unixtime {
            Ok(unix_timestamp)
        } else {
            let now = Utc::now();
            if let Ok(parsed_time) = NaiveTime::parse_from_str(&resp.result.time, "%H:%M") {
                let date_time = now.date_naive().and_time(parsed_time);
                let timestamp = date_time.timestamp();
                return Ok(timestamp);
            }
            Ok(now.timestamp())
        }
    }

    /// Returns the get location of this [`Gen2DeviceClient`].
    /// Calls the Sys.GetConfig endpoint to retrieve the location.
    /// See: https://shelly-api-docs.shelly.cloud/gen2/ComponentsAndServices/Sys#sysgetconfig
    pub async fn get_location(&self) -> Result<(f64, f64), ShellyRpcError> {
        trace!("get_location");
        let resp: SysGetConfigResponse = Self::execute_rpc(
            self.address,
            &serde_json::json!({"id": 1, "method": SysMethod::GetConfig}),
        )
        .await?;
        Ok((resp.result.location.lat, resp.result.location.lon))
    }

    /// Returns the get the value associated with key from the KVS of this [`Gen2DeviceClient`].
    /// Calls the KVS.Get endpoint to retrieve the value associated with the key.
    /// See: https://shelly-api-docs.shelly.cloud/gen2/ComponentsAndServices/KVS#kvsget
    pub async fn get_value(&self, key: &str) -> Result<String, ShellyRpcError> {
        trace!("get_value '{key}'");
        let resp: KeyValueStoreGetResponse = Self::execute_rpc(
            self.address,
            &serde_json::json!({"id": 1, "method": KeyValueStoreMethod::Get, "params": { "key": key}}),
        )
        .await?;

        Ok(resp.result.value)
    }

    /// Sets a value associated with the given key in the KVS of this [`Gen2DeviceClient`].
    /// Calls the KVS.Set endpoint to store the value with the given key.
    /// See: https://shelly-api-docs.shelly.cloud/gen2/ComponentsAndServices/KVS#kvsset
    pub async fn set_value(&self, key: &str, value: &str) -> Result<u32, ShellyRpcError> {
        trace!("set_value '{key}': '{value}'");
        let resp: KeyValueStoreSetResponse = Self::execute_rpc(
            self.address,
            &serde_json::json!({"id": 1, "method": KeyValueStoreMethod::Set, "params": { "key": key, "value": value}}),
        )
        .await?;

        Ok(resp.result.rev)
    }

    async fn execute_rpc<T, R>(address: &str, body: &T) -> Result<R, ShellyRpcError>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        trace!(
            "execute_rpc<T, R>(address: {address}, body: {:?})",
            serde_json::to_string(body)
        );
        let res_body = reqwest::Client::new()
            .post(format!("http://{address}/rpc"))
            .json(body)
            .send()
            .await?
            .text()
            .await?;

        match serde_json::from_str(&res_body) {
            Ok(r) => Ok(r),
            Err(outer) => match serde_json::from_str(&res_body) {
                Ok(e) => Err(ShellyRpcError::HttpApiError(e)),
                Err(inner) => Err(ShellyRpcError::SerdeJsonBiError(outer, inner)),
            },
        }
    }
}
