use serde::Deserialize;

/// Successful Login response
#[derive(Deserialize, Debug)]
pub struct TwitchLoginSuccessResponse {
    pub code: String,
    pub scope: String,
    pub state: String,
}

/// Failed Login struct
#[derive(Deserialize, Debug)]
pub struct TwitchLoginFailResponse {
    pub error: String,
    pub error_description: String,
    pub state: String,
}
