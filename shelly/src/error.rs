use crate::data::ShellyError;
use std::error::Error;

#[derive(Debug)]
pub enum ShellyRpcError {
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
    SerdeJsonBiError(serde_json::Error, serde_json::Error),
    HttpApiError(ShellyError),
}

impl Error for ShellyRpcError {}

// Implement the Error trait for the custom error type
// code: -103, message: "Invalid argument 'key': length should be less than 42!"
impl std::fmt::Display for ShellyRpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Implement the Display trait for the custom error type
        match self {
            ShellyRpcError::ReqwestError(e) => write!(f, "Reqwest error: {}", e),
            ShellyRpcError::SerdeJsonError(e) => write!(f, "serde_json error: {}", e),
            ShellyRpcError::SerdeJsonBiError(outer, inner) => {
                write!(f, "serde_json error: (outer: {}, inner: {})", outer, inner)
            }
            ShellyRpcError::HttpApiError(e) => write!(
                f,
                "Shelly API error: (code: {}, message: {})",
                e.error.code, e.error.message
            ),
        }
    }
}

impl From<reqwest::Error> for ShellyRpcError {
    fn from(err: reqwest::Error) -> Self {
        ShellyRpcError::ReqwestError(err)
    }
}

impl From<serde_json::Error> for ShellyRpcError {
    fn from(err: serde_json::Error) -> Self {
        ShellyRpcError::SerdeJsonError(err)
    }
}

impl From<ShellyError> for ShellyRpcError {
    fn from(err: ShellyError) -> Self {
        ShellyRpcError::HttpApiError(err)
    }
}
