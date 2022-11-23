use std::{path::PathBuf, time::Duration};

type Result<T> = anyhow::Result<T>;

const TIMEOUT: Duration = Duration::from_secs(5);

fn main() -> Result<()> {
    let authorized_keys_location = authorized_keys_location()?;
    println!("ssh dir is: {}\n", authorized_keys_location.display());

    let urls = vec![
        "https://github.com/konradmalik.keys",
        "https://gitlab.com/konradmalik.keys",
    ];

    for url in urls {
        println!("getting keys from {}\n", url);

        let keys = url_contents_to_string(url, TIMEOUT)?;

        println!("{}\n", keys);
    }
    Ok(())
}

pub fn url_contents_to_string(url: &str, timeout: Duration) -> Result<String> {
    let req = ureq::get(url).timeout(timeout).call()?;
    req.into_string().map_err(anyhow::Error::from)
}

pub fn authorized_keys_location() -> Result<PathBuf> {
    let relative_authorized_keys_path = PathBuf::from(".ssh").join("authorized_keys");
    directories::UserDirs::new()
        .ok_or_else(|| anyhow::format_err!("cannot get current user dirs"))
        .map(|ud| ud.home_dir().join(relative_authorized_keys_path))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_authorized_keys_location() {
        let authorized_keys = authorized_keys_location();
        assert!(
            authorized_keys.is_ok(),
            "authorized_keys location must be returned"
        );
    }
}
