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
    req.into_string()
        .map(|s| s.trim().to_owned())
        .map_err(anyhow::Error::from)
}

fn filter_key(s: &str, filter_prefix: &str) -> bool {
    s.starts_with(filter_prefix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_key() {
        let prefix = "ssh-";
        let lines = vec![
            ("something", false),
            ("ssh-key", true),
            ("ssh-key lelele test", true),
            ("key lelele test", false),
            ("# comment key", false),
        ];
        for line in lines {
            assert!(
                filter_key(line.0, prefix) == line.1,
                "filter on {} must be {}",
                line.0,
                line.1
            );
        }
    }
}
