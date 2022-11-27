use crate::Result;
use std::time::Duration;
use url::Url;

const TIMEOUT: Duration = Duration::from_secs(5);

// TODO validate if keys are returned

pub fn url_to_keys(url: &Url) -> Result<String> {
    let url_contents = url_contents_to_string(url)?;
    Ok(format!("# keys from {}\n{}", url, url_contents))
}

fn url_contents_to_string(url: &Url) -> Result<String> {
    let req = ureq::get(url.as_str()).timeout(TIMEOUT).call()?;
    req.into_string().map_err(anyhow::Error::from)
}
