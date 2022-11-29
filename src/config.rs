use std::path::PathBuf;
use std::time::Duration;

use anyhow::anyhow;
use url::Url;

use crate::cli::Args;
use crate::Result;

#[derive(Debug)]
pub struct Config {
    pub destination: Option<PathBuf>,
    pub sources: Vec<Url>,
    pub timeout: Duration,
}

impl Config {
    pub fn new_from_args(args: Args) -> Result<Self> {
        let destination_path = args.destination.map(PathBuf::from);
        let mut sources_urls: Vec<Url> = Vec::new();
        for source in args.sources {
            let source_url = Url::parse(&source)?;
            sources_urls.push(source_url);
        }

        let config = Config {
            destination: destination_path,
            sources: sources_urls,
            timeout: Duration::from_secs(args.timeout),
        };

        match config.validate() {
            Ok(_) => Ok(config),
            Err(e) => Err(e),
        }
    }

    fn validate(&self) -> Result<()> {
        if self.sources.is_empty() {
            Err(anyhow!("sources must not be empty"))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_args() {
        let mut args = Args {
            destination: None,
            sources: vec!["https://example.com".to_owned()],
            timeout: 12,
        };
        let mut config = Config::new_from_args(args).unwrap();
        assert!(config.destination.is_none());
        assert!(config.sources.len() == 1);
        assert!(config.timeout.as_secs() == 12);

        args = Args {
            destination: Some("/tmp/file".to_owned()),
            sources: vec!["https://example.com".to_owned()],
            timeout: 12,
        };
        config = Config::new_from_args(args).unwrap();
        assert!(config.destination.is_some());
        assert!(config.sources.len() == 1);
        assert!(config.timeout.as_secs() == 12);
    }

    #[test]
    fn test_config_validation() {
        let args = Args {
            destination: None,
            sources: vec![],
            timeout: 12,
        };
        assert!(Config::new_from_args(args).is_err())
    }
}
