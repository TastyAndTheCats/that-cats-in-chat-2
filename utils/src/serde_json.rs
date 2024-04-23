use reqwest::{Error, Response};
use serde_json::{from_str, Value};

pub async fn unwrap_reqwest(reqwest_response_result: Result<Response, Error>) -> Value {
    from_str(
        &reqwest_response_result
            .unwrap()
            .text()
            .await
            .unwrap_or(String::from("")),
    )
    .expect("Unitelligible Reqwest response")
}
