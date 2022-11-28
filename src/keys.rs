use crate::Result;
use std::time::Duration;
use url::Url;

pub fn url_to_keys(url: &Url, filter_prefix: &str, timeout: &Duration) -> Result<String> {
    let url_contents = url_contents_to_string(url, timeout)?
        .lines()
        .filter(|l| filter_key(l, filter_prefix))
        .fold(String::new(), |s, l| s + l + "\n");
    Ok(format!("# keys from {}\n{}\n", url, url_contents))
}

fn url_contents_to_string(url: &Url, timeout: &Duration) -> Result<String> {
    let req = ureq::get(url.as_str()).timeout(timeout.to_owned()).call()?;
    req.into_string().map_err(anyhow::Error::from)
}

fn filter_key(s: &str, filter_prefix: &str) -> bool {
    s.starts_with(filter_prefix)
}
