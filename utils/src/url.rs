use url::Url;

pub fn construct_url(base_url: &str, params: Vec<(&str, &str)>) -> String {
    Url::parse_with_params(base_url, &params)
        .expect("URL wasn't able to be formed")
        .as_str()
        .to_owned()
}
