use serde::{Deserialize, Serialize};

//------------------------------
// Schedule endpoint
//------------------------------

#[derive(Debug, Serialize)]
pub enum ScheduleMethod {
    #[serde(rename = "Schedule.Create")]
    Create,

    #[serde(rename = "Schedule.Update")]
    Update,

    #[serde(rename = "Schedule.List")]
    List,
}

#[derive(Debug, Deserialize)]
pub struct ScheduleCreateResponse {
    pub id: u8,
    pub src: String,
    pub result: ScheduleCreateResponseResult,
}

#[derive(Debug, Deserialize)]
pub struct ScheduleCreateResponseResult {
    pub id: u32,
    pub rev: u32,
}

#[derive(Debug, Deserialize)]
pub struct ScheduleUpdateResponse {
    pub id: u8,
    pub src: String,
    pub result: ScheduleUpdateResponseResult,
}

#[derive(Debug, Deserialize)]
pub struct ScheduleUpdateResponseResult {
    pub rev: u32,
}

#[derive(Debug, Deserialize)]
pub struct ScheduleListResponse {
    pub id: u8,
    pub src: String,
    pub result: ScheduleListResponseResult,
}

#[derive(Debug, Deserialize)]
pub struct ScheduleListResponseResult {
    pub jobs: Vec<ScheduleJobWithOptionalId>,
    pub rev: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ScheduleJobWithOptionalId {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    pub enable: bool,
    pub timespec: String,
    pub calls: Vec<ScheduleJobMethod>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ScheduleJobMethod {
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

//------------------------------
// KVS endpoint
//------------------------------

#[derive(Debug, Serialize)]
pub enum KeyValueStoreMethod {
    #[serde(rename = "KVS.Set")]
    Set,

    #[serde(rename = "KVS.Get")]
    Get,
}

#[derive(Debug, Deserialize)]
pub struct KeyValueStoreGetResponse {
    pub id: u8,
    pub src: String,
    pub result: KeyValueStoreGetResponseResult,
}

#[derive(Debug, Deserialize)]
pub struct KeyValueStoreSetResponse {
    pub id: u8,
    pub src: String,
    pub result: KeyValueStoreSetResponseResult,
}

#[derive(Debug, Deserialize)]
pub struct KeyValueStoreGetResponseResult {
    pub etag: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct KeyValueStoreSetResponseResult {
    pub etag: String,
    pub rev: u32,
}

//------------------------------
// System endpoint
//------------------------------

#[derive(Debug, Serialize)]
pub enum SysMethod {
    #[serde(rename = "Sys.GetConfig")]
    GetConfig,

    #[serde(rename = "Sys.GetStatus")]
    GetStatus,
}

#[derive(Debug, Deserialize)]
pub struct SysGetConfigResponse {
    pub id: u8,
    pub src: String,
    pub result: SysGetConfigResponseResult,
}

#[derive(Debug, Deserialize)]
pub struct SysGetConfigResponseResult {
    pub location: Location,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub tz: String,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Deserialize)]
pub struct SysGetStatusResponse {
    pub id: u8,
    pub src: String,
    pub result: SysGetStatusResponseResult,
}

#[derive(Debug, Deserialize)]
pub struct SysGetStatusResponseResult {
    pub time: String,
    pub unixtime: Option<i64>,
}

//------------------------------
// Generic error type
//------------------------------
#[derive(Debug, Deserialize)]
pub struct ShellyError {
    pub id: u8,
    pub src: String,
    pub error: ShellyErrorCodeWithMessage,
}

#[derive(Debug, Deserialize)]
pub struct ShellyErrorCodeWithMessage {
    pub code: i8,
    pub message: String,
}

// Key not found in the KVS.
pub const KEY_NOT_FOUND: i8 = -105;
