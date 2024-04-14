//! Functions that use the Url crate or otherwise are related to URI/Ls
use url::Url;

/// Composes a url using the URL crate from a base url and some params
pub fn construct_url(base_url: &str, params: Vec<(&str, &str)>) -> String {
    Url::parse_with_params(base_url, &params)
        .expect("URL wasn't able to be formed")
        .as_str()
        .to_owned()
}

/// Formats a Vec into a HTTP POST request body
pub fn compose_post_body(params: Vec<(&str, String)>) -> String {
    let mut output: Vec<String> = vec![];
    for param in params.into_iter() {
        output.push(format!("{}={}", param.0, param.1));
    }
    output.join("&")
}
