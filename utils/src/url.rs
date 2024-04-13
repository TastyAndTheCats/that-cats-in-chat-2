use url::Url;

pub fn construct_url(base_url: &str, params: Vec<(&str, &str)>) -> String {
    Url::parse_with_params(base_url, &params)
        .expect("URL wasn't able to be formed")
        .as_str()
        .to_owned()
}

pub fn compose_post_body(params: Vec<(&str, String)>) -> String {
    let mut output: Vec<String> = vec![];
    for param in params.into_iter() {
        output.push(format!("{}={}", param.0, param.1));
    }
    output.join("&")
}
