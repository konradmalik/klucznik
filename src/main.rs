use std::{env, path::PathBuf, time::Duration};

use config::Config;
use url::Url;
mod config;

type Result<T> = anyhow::Result<T>;

const TIMEOUT: Duration = Duration::from_secs(5);

// test: cargo run -- /tmp/auth https://github.com/konradmalik.keys https://gitlab.com/konradmalik.keys
fn main() -> Result<()> {
    let config = parse_args(env::args())?;
    println!("config is: {:?}\n", config);
    println!("destination is {}", config.destination.display());
    println!("sources are {:?}", config.sources);

    let authorized_keys_location = default_authorized_keys_location()?;
    println!("ssh dir is: {}\n", authorized_keys_location.display());

    for url in config.sources {
        println!("getting keys from {}\n", url);

        let keys = url_contents_to_string(&url, TIMEOUT)?;

        println!("{}\n", keys);
    }
    Ok(())
}

pub fn url_contents_to_string(url: &Url, timeout: Duration) -> Result<String> {
    let req = ureq::get(url.as_str()).timeout(timeout).call()?;
    req.into_string().map_err(anyhow::Error::from)
}

pub fn default_authorized_keys_location() -> Result<PathBuf> {
    let relative_authorized_keys_path = PathBuf::from(".ssh").join("authorized_keys");
    directories::UserDirs::new()
        .ok_or_else(|| anyhow::format_err!("cannot get current user dirs"))
        .map(|ud| ud.home_dir().join(relative_authorized_keys_path))
}

pub fn parse_args(args: env::Args) -> Result<Config> {
    let args_vec: Vec<String> = args.collect();
    // TODO validate
    let destination = args_vec[1].to_owned();
    let sources = &args_vec[2..];
    Config::new_from_strings(destination, sources.to_vec())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_authorized_keys_location() {
        let authorized_keys = default_authorized_keys_location();
        assert!(
            authorized_keys.is_ok(),
            "authorized_keys location must be returned"
        );
    }
}
