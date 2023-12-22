use daylight_extender::{Controller, SCHEDULE_JOB_ID};
use mockito::Server;
use shelly::api::Gen2DeviceClient;

mod data;

#[tokio::test]
async fn successful_create() {
    // arrange
    let day_length = 12;
    let day_length_seconds = i64::from(day_length) * 60 * 60;
    let schedule_id = "1";
    let schedule_revision = 33;

    let tz = "Europe/Berlin";
    let lat = 52.516293;
    let lon = 13.377713;

    let time = "16:20";
    // Wednesday, 20 December 2023 16:20:00
    let unix_timestamp = 1703085600;
    // Wednesday, 20 December 2023 08:14:19
    let sunrise = 1703056459;
    // Wednesday, 20 December 2023 15:53:24
    let sunset = 1703084004;

    let light_on = sunset - day_length_seconds;
    let toggle_after = sunrise - light_on;

    let mut server = Server::new_async().await;
    let host = server.host_with_port();
    let get_config_mock = server
        .mock("POST", "/rpc")
        .match_body(data::mockito::match_body::GET_CONFIG)
        .with_body(data::mockito::with_body::get_config(tz, lat, lon))
        .create_async()
        .await;

    let get_status_mock = server
        .mock("POST", "/rpc")
        .match_body(data::mockito::match_body::GET_STATUS)
        .with_body(data::mockito::with_body::get_status(time, unix_timestamp))
        .create_async()
        .await;

    // get_value
    let get_value_mock = server
        .mock("POST", "/rpc")
        .match_body(data::mockito::match_body::get_value(SCHEDULE_JOB_ID).as_str())
        .with_body(data::mockito::with_body::get_value_error(SCHEDULE_JOB_ID))
        .create_async()
        .await;

    // create_schedule
    let create_schedule_mock = server
        .mock("POST", "/rpc")
        .match_body(data::mockito::match_body::create_schedule(light_on, toggle_after).as_str())
        .with_body(data::mockito::with_body::create_schedule(
            schedule_id.parse().expect("Not a valid u32"),
            schedule_revision,
        ))
        .create_async()
        .await;

    // set_value
    let set_value_mock = server
        .mock("POST", "/rpc")
        .match_body(data::mockito::match_body::set_value(SCHEDULE_JOB_ID, schedule_id).as_str())
        .with_body(data::mockito::with_body::set_value().as_str())
        .create_async()
        .await;

    let client = Gen2DeviceClient::new(&host);
    let core = Controller::new(&client);

    // act
    let actual = core.execute(day_length).await;

    // assert
    get_config_mock.assert_async().await;
    get_status_mock.assert_async().await;
    get_value_mock.assert_async().await;
    create_schedule_mock.assert_async().await;
    set_value_mock.assert_async().await;
    assert!(actual.is_ok(), "Expected Ok is Error");
    assert_eq!(schedule_revision, actual.expect("Unexpected"));
}

#[tokio::test]
async fn successful_update() {
    // arrange
    let day_length = 12;
    let day_length_seconds = i64::from(day_length) * 60 * 60;
    let schedule_id = "1";
    let schedule_revision = 35;

    let tz = "Europe/Berlin";
    let lat = 52.516293;
    let lon = 13.377713;

    let time = "16:20";
    // Wednesday, 20 December 2023 16:20:00
    let unix_timestamp = 1703085600;
    // Wednesday, 20 December 2023 08:14:19
    let sunrise = 1703056459;
    // Wednesday, 20 December 2023 15:53:24
    let sunset = 1703084004;

    let light_on = sunset - day_length_seconds;
    let toggle_after = sunrise - light_on;

    let mut server = Server::new_async().await;
    let host = server.host_with_port();
    let get_config_mock = server
        .mock("POST", "/rpc")
        .match_body(data::mockito::match_body::GET_CONFIG)
        .with_body(data::mockito::with_body::get_config(tz, lat, lon))
        .create_async()
        .await;

    let get_status_mock = server
        .mock("POST", "/rpc")
        .match_body(data::mockito::match_body::GET_STATUS)
        .with_body(data::mockito::with_body::get_status(time, unix_timestamp))
        .create_async()
        .await;

    // get_value
    let get_value_mock = server
        .mock("POST", "/rpc")
        .match_body(data::mockito::match_body::get_value(SCHEDULE_JOB_ID).as_str())
        .with_body(data::mockito::with_body::get_value(schedule_id))
        .create_async()
        .await;

    // create_schedule
    let update_schedule_mock = server
        .mock("POST", "/rpc")
        .match_body(
            data::mockito::match_body::update_schedule(
                schedule_id.parse().expect("Not a valid u32"),
                light_on,
                toggle_after,
                true,
            )
            .as_str(),
        )
        .with_body(data::mockito::with_body::update_schedule(schedule_revision))
        .create_async()
        .await;

    // set_value
    let set_value_mock = server
        .mock("POST", "/rpc")
        .match_body(data::mockito::match_body::set_value(SCHEDULE_JOB_ID, schedule_id).as_str())
        .expect(0)
        .create_async()
        .await;

    let client = Gen2DeviceClient::new(&host);
    let core = Controller::new(&client);

    // act
    let actual = core.execute(day_length).await;

    // assert
    get_config_mock.assert_async().await;
    get_status_mock.assert_async().await;
    get_value_mock.assert_async().await;
    update_schedule_mock.assert_async().await;
    set_value_mock.assert_async().await;
    assert!(actual.is_ok(), "Expected Ok is Error");
    assert_eq!(schedule_revision, actual.expect("Unexpected"));
}

#[tokio::test]
async fn successful_update_disable() {
    // arrange
    let day_length = 5;
    let day_length_seconds = i64::from(day_length) * 60 * 60;
    let schedule_id = "23";
    let schedule_revision = 37;

    let tz = "Europe/Berlin";
    let lat = 52.516293;
    let lon = 13.377713;

    let time = "16:20";
    // Wednesday, 20 December 2023 16:20:00
    let unix_timestamp = 1703085600;
    // Wednesday, 20 December 2023 08:14:19
    let sunrise = 1703056459;
    // Wednesday, 20 December 2023 15:53:24
    let sunset = 1703084004;

    let light_on = sunset - day_length_seconds;
    let toggle_after = sunrise - light_on;
    let enabled = true;

    let mut server = Server::new_async().await;
    let host = server.host_with_port();
    let get_config_mock = server
        .mock("POST", "/rpc")
        .match_body(data::mockito::match_body::GET_CONFIG)
        .with_body(data::mockito::with_body::get_config(tz, lat, lon))
        .create_async()
        .await;

    let get_status_mock = server
        .mock("POST", "/rpc")
        .match_body(data::mockito::match_body::GET_STATUS)
        .with_body(data::mockito::with_body::get_status(time, unix_timestamp))
        .create_async()
        .await;

    // get_value
    let get_value_mock = server
        .mock("POST", "/rpc")
        .match_body(data::mockito::match_body::get_value(SCHEDULE_JOB_ID).as_str())
        .with_body(data::mockito::with_body::get_value(schedule_id))
        .create_async()
        .await;

    // list_schedule
    let list_schedule_mock = server
        .mock("POST", "/rpc")
        .match_body(data::mockito::match_body::LIST_SCHEDULE)
        .with_body(data::mockito::with_body::list_schedule(
            schedule_id.parse().expect("Not a valid u32"),
            light_on,
            toggle_after,
            enabled,
            schedule_revision,
        ))
        .create_async()
        .await;

    // update_schedule
    let disable_schedule_mock = server
        .mock("POST", "/rpc")
        .match_body(
            data::mockito::match_body::disable_schedule(
                schedule_id.parse().expect("Not a valid u32"),
            )
            .as_str(),
        )
        .with_body(data::mockito::with_body::update_schedule(schedule_revision))
        .create_async()
        .await;

    // don't call set_value
    let set_value_mock = server
        .mock("POST", "/rpc")
        .match_body(data::mockito::match_body::set_value(SCHEDULE_JOB_ID, schedule_id).as_str())
        .expect(0)
        .create_async()
        .await;

    let client = Gen2DeviceClient::new(&host);
    let core = Controller::new(&client);

    // act
    let actual = core.execute(day_length).await;

    // assert
    get_config_mock.assert_async().await;
    get_status_mock.assert_async().await;
    get_value_mock.assert_async().await;
    list_schedule_mock.assert_async().await;
    disable_schedule_mock.assert_async().await;
    set_value_mock.assert_async().await;
    assert!(actual.is_ok(), "Expected Ok is Error");
    assert_eq!(schedule_revision, actual.expect("Unexpected"));
}

#[tokio::test]
async fn successful_update_no_action() {
    // arrange
    let day_length = 4;
    let day_length_seconds = i64::from(day_length) * 60 * 60;
    let schedule_id = "17";
    let schedule_revision = 35;

    let tz = "Europe/Berlin";
    let lat = 52.516293;
    let lon = 13.377713;

    let time = "16:20";
    // Wednesday, 20 December 2023 16:20:00
    let unix_timestamp = 1703085600;
    // Wednesday, 20 December 2023 08:14:19
    let sunrise = 1703056459;
    // Wednesday, 20 December 2023 15:53:24
    let sunset = 1703084004;

    let light_on = sunset - day_length_seconds;
    let toggle_after = sunrise - light_on;
    let enabled = false;

    let mut server = Server::new_async().await;
    let host = server.host_with_port();
    let get_config_mock = server
        .mock("POST", "/rpc")
        .match_body(data::mockito::match_body::GET_CONFIG)
        .with_body(data::mockito::with_body::get_config(tz, lat, lon))
        .create_async()
        .await;

    let get_status_mock = server
        .mock("POST", "/rpc")
        .match_body(data::mockito::match_body::GET_STATUS)
        .with_body(data::mockito::with_body::get_status(time, unix_timestamp))
        .create_async()
        .await;

    // get_value
    let get_value_mock = server
        .mock("POST", "/rpc")
        .match_body(data::mockito::match_body::get_value(SCHEDULE_JOB_ID).as_str())
        .with_body(data::mockito::with_body::get_value(schedule_id))
        .create_async()
        .await;

    // list_schedule
    let list_schedule_mock = server
        .mock("POST", "/rpc")
        .match_body(data::mockito::match_body::LIST_SCHEDULE)
        .with_body(data::mockito::with_body::list_schedule(
            schedule_id.parse().expect("Not a valid u32"),
            light_on,
            toggle_after,
            enabled,
            schedule_revision,
        ))
        .create_async()
        .await;

    // don't call update_schedule
    let update_schedule_mock = server
        .mock("POST", "/rpc")
        .match_body(
            data::mockito::match_body::update_schedule(
                schedule_id.parse().expect("Not a valid u32"),
                light_on,
                toggle_after,
                enabled,
            )
            .as_str(),
        )
        .expect(0)
        .create_async()
        .await;

    // don't call set_value
    let set_value_mock = server
        .mock("POST", "/rpc")
        .match_body(data::mockito::match_body::set_value(SCHEDULE_JOB_ID, schedule_id).as_str())
        .expect(0)
        .create_async()
        .await;

    let client = Gen2DeviceClient::new(&host);
    let core = Controller::new(&client);

    // act
    let actual = core.execute(day_length).await;

    // assert
    get_config_mock.assert_async().await;
    get_status_mock.assert_async().await;
    get_value_mock.assert_async().await;
    list_schedule_mock.assert_async().await;
    update_schedule_mock.assert_async().await;
    set_value_mock.assert_async().await;
    assert!(actual.is_ok(), "Expected Ok is Error");
    assert_eq!(schedule_revision, actual.expect("Unexpected"));
}
