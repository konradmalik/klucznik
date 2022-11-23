use std::{path::PathBuf, time::Duration};

type Result<T> = anyhow::Result<T>;

const TIMEOUT: Duration = Duration::from_secs(5);

fn main() -> Result<()> {
    let ssh_dir = ssh_user_dir_location()?;
    println!("ssh dir is: {}\n", ssh_dir.display());

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

pub fn ssh_user_dir_location() -> Result<PathBuf> {
    directories::UserDirs::new()
        .ok_or(anyhow::format_err!("cannot get HOME dir for current user"))
        .map(|ud| ud.home_dir().join(".ssh"))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_user_ssh_dir() {
        let ssh_dir = ssh_user_dir_location();
        assert!(ssh_dir.is_ok(), "ssh dir must be returned");
        assert!(
            ssh_dir.unwrap().is_dir(),
            "ssh dir must exist and be a directory"
        );
    }
}
