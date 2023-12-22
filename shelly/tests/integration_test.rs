use mockito::Server;
use shelly::api::Gen2DeviceClient;

#[tokio::test]
async fn get_time() {
    // arrange
    let time = "16:20";
    let unix_timestamp = 1654694407;

    let expected_body = r#"{"id":1,"method":"Sys.GetStatus"}"#;

    let mock_body = serde_json::json!({
      "id": 1,
      "src": "shellyplus2pm-a8032ab636ec",
      "result": {
        "mac": "A8032AB636EC",
        "restart_required": false,
        "time": time,
        "unixtime": unix_timestamp,
        "uptime": 2339,
        "ram_size": 253464,
        "ram_free": 146012,
        "fs_size": 458752,
        "fs_free": 212992,
        "cfg_rev": 10,
        "kvs_rev": 277,
        "schedule_rev": 0,
        "webhook_rev": 0,
        "available_updates": {
          "stable": {
            "version": "0.10.2"
          }
        }
      }
    });

    let mut server = Server::new_async().await;
    let host = server.host_with_port();
    let mock = server
        .mock("POST", "/rpc")
        .match_body(expected_body.to_string().as_str())
        .with_body(mock_body.to_string())
        .create_async()
        .await;

    let uut = Gen2DeviceClient::new(&host);

    // act
    let result = uut.get_time().await.unwrap();

    // assert
    mock.assert_async().await;
    assert_eq!(unix_timestamp, result);
}

#[tokio::test]
async fn get_location() {
    // arrange
    let tz = "Europe/Sofia";
    let lat = 42.6534;
    let lon = 23.31119;

    let expected_body = r#"{"id":1,"method":"Sys.GetConfig"}"#;

    let mock_body = serde_json::json!({
      "id": 1,
      "src": "shellyplus2pm-a8032ab636ec",
      "result": {
        "device": {
          "name": null,
          "mac": "A8032AB636EC",
          "fw_id": "20220527-091739/0.10.2-beta4-gecc3a61",
          "eco_mode": false,
          "profile": "cover",
          "discoverable": false
        },
        "location": {
          "tz": tz,
          "lat": lat,
          "lon": lon
        },
        "debug": {
          "mqtt": {
            "enable": false
          },
          "websocket": {
            "enable": false
          },
          "udp": {
            "addr": null
          }
        },
        "ui_data": {},
        "rpc_udp": {
          "dst_addr": null,
          "listen_port": null
        },
        "sntp": {
          "server": "time.google.com"
        },
        "cfg_rev": 10
      }
    });

    let mut server = Server::new_async().await;
    let host = server.host_with_port();
    let mock = server
        .mock("POST", "/rpc")
        .match_body(expected_body.to_string().as_str())
        .with_body(mock_body.to_string())
        .create_async()
        .await;

    let uut = Gen2DeviceClient::new(&host);

    // act
    let (latitude, longitude) = uut.get_location().await.unwrap();

    // assert
    mock.assert_async().await;
    assert_eq!(lat, latitude);
    assert_eq!(lon, longitude);
}

#[tokio::test]
async fn get_value() {
    // arrange
    let key = "test.key";
    let value = "42";

    let expected_body = serde_json::json!({
        "id": 1,
        "method": "KVS.Get",
        "params": {
            "key": key
        }
    });

    let mock_body = serde_json::json!({
      "id": 1,
      "src": "shellyplus1-a8032abe54dc",
      "result": {
        "etag": "0DWty8HwCB",
        "value": value
      }
    });

    let mut server = Server::new_async().await;
    let host = server.host_with_port();
    let mock = server
        .mock("POST", "/rpc")
        .match_body(expected_body.to_string().as_str())
        .with_body(mock_body.to_string())
        .create_async()
        .await;

    let uut = Gen2DeviceClient::new(&host);

    // act
    let result = uut.get_value(key).await.unwrap();

    // assert
    mock.assert_async().await;
    assert_eq!(value, result);
}

#[tokio::test]
async fn set_value() {
    // arrange
    let key = "test.key";
    let value = r#"{ "object": { "id": 0, items: [ "cats", "vans", "bags"] } }"#;
    let rev = 2733;

    let expected_body = serde_json::json!({
        "id": 1,
        "method": "KVS.Set",
        "params": {
            "key": key,
            "value": value
        }
    });

    let mock_body = serde_json::json!({
      "id": 1,
      "src": "shellyplus1-a8032abe54dc",
      "result": {
        "etag": "0DWty8HwCB",
        "rev": rev
      }
    });

    let mut server = Server::new_async().await;
    let host = server.host_with_port();
    let mock = server
        .mock("POST", "/rpc")
        .match_body(expected_body.to_string().as_str())
        .with_body(mock_body.to_string())
        .create_async()
        .await;

    let uut = Gen2DeviceClient::new(&host);

    // act
    let result = uut.set_value(key, value).await.unwrap();

    // assert
    mock.assert_async().await;
    assert_eq!(rev, result);
}

#[tokio::test]
async fn connection_refused() {
    // arrange
    let host = "localhost";

    let uut = Gen2DeviceClient::new(&host);

    // act
    let result = uut.list_schedule().await;

    // assert
    assert!(result.is_err(), "Expected Error is Ok");
    println!("{}", result.err().unwrap());
}

#[tokio::test]
async fn key_too_long() {
    // arrange
    let key_too_long = "this.key.is.much.longer.than.the.allowed.forty.two.characters";

    let body = serde_json::json!({
      "id": 1,
      "src": "shellyplus1-a8032abe54dc",
      "error": {
        "code": -103,
        "message": "Invalid argument 'key': length should be less than 42!",
      }
    });

    let mut server = Server::new_async().await;
    let host = server.host_with_port();
    let mock = server
        .mock("POST", "/rpc")
        .with_body(body.to_string())
        .create_async()
        .await;

    let uut = Gen2DeviceClient::new(&host);

    // act
    let result = uut.get_value(key_too_long).await;

    // assert
    mock.assert_async().await;
    assert!(result.is_err(), "Expected Error is Ok");
    println!("{}", result.err().unwrap());
}

#[tokio::test]
async fn key_not_found() {
    // arrange
    let key_not_found = "key.not.found";

    let body = serde_json::json!({
      "id": 1,
      "src": "shellyplus1-a8032abe54dc",
      "error": {
        "code": -105,
        "message": "Argument 'key', value 'item1' not found !",
      }
    });

    let mut server = Server::new_async().await;
    let host = server.host_with_port();
    let mock = server
        .mock("POST", "/rpc")
        .with_body(body.to_string())
        .create_async()
        .await;

    let uut = Gen2DeviceClient::new(&host);

    // act
    let result = uut.get_value(key_not_found).await;

    // assert
    mock.assert_async().await;
    assert!(result.is_err(), "Expected Error is Ok");
    println!("{}", result.err().unwrap());
}
