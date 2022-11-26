use std::path::PathBuf;

use url::Url;

use crate::Result;

#[derive(Debug)]
pub struct Config {
    pub destination: PathBuf,
    pub sources: Vec<Url>,
}

impl Config {
    pub fn new_from_strings(destination: String, sources: Vec<String>) -> Result<Self> {
        let destination_path = PathBuf::from(destination);
        let mut sources_urls: Vec<Url> = Vec::new();
        for source in sources {
            let source_url = Url::parse(&source)?;
            sources_urls.push(source_url);
        }

        Ok(Config {
            destination: destination_path,
            sources: sources_urls,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_config_from_args() {
        assert!(true, "testing test");
    }
}
