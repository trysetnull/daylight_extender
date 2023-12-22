pub mod mockito {

    pub mod match_body {
        use chrono::{Local, TimeZone, Timelike};

        pub const GET_CONFIG: &str = r#"{"id":1,"method":"Sys.GetConfig"}"#;
        pub const GET_STATUS: &str = r#"{"id":1,"method":"Sys.GetStatus"}"#;
        pub const LIST_SCHEDULE: &str = r#"{"id":1,"method":"Schedule.List"}"#;

        pub fn get_value(key: &str) -> String {
            serde_json::json!({
                "id":1,
                "method":"KVS.Get",
                "params":{
                    "key": key
                }
            })
            .to_string()
        }
        pub fn set_value(key: &str, value: &str) -> String {
            serde_json::json!({
                "id":1,
                "method":"KVS.Set",
                "params":{
                    "key": key,
                    "value": value
                }
            })
            .to_string()
        }

        pub fn create_schedule(light_on: i64, toggle_after: i64) -> String {
            let light_on_dt = Local.timestamp_opt(light_on, 0).unwrap();
            serde_json::json!({
                "id":1,
                "method":"Schedule.Create",
                "params":{
                    "enable": true,
                    "timespec": format!("{} {} {} * * 0,1,2,3,4,5,6", light_on_dt.second(), light_on_dt.minute(), light_on_dt.hour()),
                    "calls":[{
                        "method":"switch.set",
                        "params":{
                            "on": true,
                            "toggle_after": toggle_after,
                            "id":0
                        }
                    }]
                }
            })
            .to_string()
        }

        pub fn disable_schedule(id: u32) -> String {
            serde_json::json!({
                "id":1,
                "method":"Schedule.Update",
                "params":{
                    "id": id,
                    "enable": false,
                }
            })
            .to_string()
        }

        pub fn update_schedule(id: u32, light_on: i64, toggle_after: i64, enabled: bool) -> String {
            let light_on_dt = Local.timestamp_opt(light_on, 0).unwrap();
            serde_json::json!({
                "id":1,
                "method":"Schedule.Update",
                "params":{
                    "id": id,
                    "enable": enabled,
                    "timespec": format!("{} {} {} * * 0,1,2,3,4,5,6", light_on_dt.second(), light_on_dt.minute(), light_on_dt.hour()),
                    "calls":[{
                        "method":"switch.set",
                        "params":{
                            "on": true,
                            "toggle_after": toggle_after,
                            "id": 0
                        }
                    }]
                }
            })
            .to_string()
        }
    }

    pub mod with_body {
        use chrono::{Local, TimeZone, Timelike};

        pub fn get_config(tz: &str, lat: f64, lon: f64) -> String {
            serde_json::json!({
                "id": 1,
                "src": "shelly-test-data",
                "result": {
                    "location": {
                        "tz": tz,
                        "lat": lat,
                        "lon": lon
                    },
                }
            })
            .to_string()
        }

        pub fn get_status(time: &str, unix_timestamp: i64) -> String {
            serde_json::json!({
                "id": 1,
                "src": "shelly-test-data",
                "result": {
                    "time": time,
                    "unixtime": unix_timestamp,
                }
            })
            .to_string()
        }

        pub fn get_value(value: &str) -> String {
            serde_json::json!({
                "id": 1,
                "src": "shelly-test-data",
                "result": {
                    "etag": "D34dB3Ef",
                    "value": value,
                }
            })
            .to_string()
        }

        pub fn get_value_error(key: &str) -> String {
            serde_json::json!({
                "id":1,
                "src":"shelly-test-data",
                "error":{
                    "code":-105,
                    "message":format!("Argument 'key', value '{key}' not found!")
                }
            })
            .to_string()
        }

        pub fn set_value() -> String {
            serde_json::json!({
                "id": 1,
                "src": "shelly-test-data",
                "result": {
                    "etag": "D34dB3Ef",
                    "rev": 42,
                }
            })
            .to_string()
        }

        pub fn create_schedule(id: u32, rev: u32) -> String {
            serde_json::json!({
                "id": 1,
                "src": "shelly-test-data",
                "result": {
                    "id": id,
                    "rev": rev
                }
            })
            .to_string()
        }

        pub fn update_schedule(rev: u32) -> String {
            serde_json::json!({
                "id": 1,
                "src": "shelly-test-data",
                "result": {
                    "rev": rev
                }
            })
            .to_string()
        }

        pub fn list_schedule(
            id: u32,
            light_on: i64,
            toggle_after: i64,
            enabled: bool,
            rev: u32,
        ) -> String {
            let light_on_dt = Local.timestamp_opt(light_on, 0).unwrap();
            serde_json::json!({
                "id":1,
                "src":"shelly-test-data",
                "result": {
                    "jobs": [{
                        "id": id,
                        "enable": enabled,
                        "timespec": format!("{} {} {} * * 0,1,2,3,4,5,6", light_on_dt.second(), light_on_dt.minute(), light_on_dt.hour()),
                        "calls":[{
                            "method": "switch.set",
                            "params": {
                                "on": true,
                                "toggle_after": toggle_after,
                                "id": 0
                            }
                        }]
                    }],
                    "rev": rev
                }
            })
            .to_string()
        }
    }
}
