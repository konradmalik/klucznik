use crate::config::Config;
use std::io::{self, Write};
mod cli;
mod config;
mod files;
mod keys;

type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    let args = cli::parse_args();

    let config = Config::new_from_args(args)?;

    for url in config.sources {
        let authorized_keys = keys::url_to_keys(&url, &config.filter_prefix, &config.timeout)?;

        match config.destination {
            Some(ref dest) => files::write_keys_to_file(authorized_keys.as_str(), dest)?,
            None => io::stdout().write_all(authorized_keys.as_bytes())?,
        }
    }
    Ok(())
}
