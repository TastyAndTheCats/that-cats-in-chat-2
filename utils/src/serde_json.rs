use reqwest::{Error, Response};
use serde_json::{from_str, Value};

pub async fn unwrap_reqwest(reqwest_response_result: Result<Response, Error>) -> Value {
    let default = Value::String("Unitelligible Reqwest response".to_owned());
    from_str(
        &reqwest_response_result
            .unwrap()
            .text()
            .await
            .unwrap_or(String::from("")),
    )
    .unwrap_or(default)
}
