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

    let mut strings = Vec::new();
    for url in config.sources {
        let pub_keys = keys::url_to_keys(&url, &config.timeout)?;
        let pub_keys_string = keys::keys_to_string(&url, pub_keys)?;
        strings.push(pub_keys_string);
    }

    let authorized_keys = concatenate_key_strings(strings);
    match config.destination {
        Some(ref dest) => files::write_keys_to_file(&authorized_keys, dest)?,
        None => io::stdout().write_all(authorized_keys.as_bytes())?,
    }
    Ok(())
}

fn concatenate_key_strings(strs: Vec<String>) -> String {
    let mut buf = String::new();
    for s in strs {
        buf.push_str(&format!("{}\n", s));
    }
    buf
}
