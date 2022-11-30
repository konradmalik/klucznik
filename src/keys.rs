use crate::Result;
use anyhow::anyhow;
use std::fmt;
use std::time::Duration;
use url::Url;

// from https://gitlab.com/-/profile/keys
const ALGORITHMS: &[&str] = &[
    "ssh-rsa",
    "ssh-dss",
    "ecdsa-sha2-nistp256",
    "ecdsa-sha2-nistp384",
    "ecdsa-sha2-nistp521",
    "ssh-ed25519",
    "sk-ecdsa-sha2-nistp256@openssh.com",
    "sk-ssh-ed25519@openssh.com",
];

pub struct PublicSSHKey {
    pub algorithm: String,
    // base64
    pub key: String,
    pub comment: String,
}

impl PublicSSHKey {
    pub fn maybe_from_string(s: &str) -> Result<Self> {
        let elems: Vec<&str> = s.split_whitespace().collect();
        if elems.len() < 2 {
            return Err(anyhow!(
                "cannot create PublicSSHKey from less than 2 elements"
            ));
        }
        let algo = elems[0];
        if !Self::valid_algo(algo) {
            return Err(anyhow!(
                "cannot interpret algorithm that does not match with one of the allowed ones"
            ));
        }

        let key = elems[1];
        let comment = elems
            .get(2..)
            .map(|x| x.join(" "))
            .unwrap_or_else(|| "".to_string());

        Ok(PublicSSHKey {
            algorithm: algo.trim().to_string(),
            key: key.trim().to_string(),
            comment: comment.trim().to_string(),
        })
    }

    fn valid_algo(s: &str) -> bool {
        for alg in ALGORITHMS {
            if s.starts_with(alg) {
                return true;
            }
        }
        false
    }
}

impl fmt::Display for PublicSSHKey {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let key = format!("{} {} {}", &self.algorithm, &self.key, &self.comment);
        fmt.write_str(key.trim())
    }
}

pub fn url_to_keys(url: &Url, timeout: &Duration) -> Result<String> {
    let url_contents = url_contents_to_string(url, timeout)?;
    let keys = url_contents.lines().flat_map(|l| {
        PublicSSHKey::maybe_from_string(l).map_err(|e| {
            eprintln!("{}", e);
            e
        })
    });
    let keys_string = keys.fold(String::new(), |s, l| s + l.to_string().as_str() + "\n");
    Ok(format!("# keys from {}\n{}\n", url, keys_string))
}

fn url_contents_to_string(url: &Url, timeout: &Duration) -> Result<String> {
    let req = ureq::get(url.as_str()).timeout(timeout.to_owned()).call()?;
    req.into_string()
        .map(|s| s.trim().to_owned())
        .map_err(anyhow::Error::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_creation() {
        let lines = vec![
            ("something", false),
            ("ssh-key abc comment", true),
            ("ssh-key abc comment test", true),
            ("ssh-key abc", true),
            ("key lelele test", false),
            ("ssh lelele test", false),
            ("# comment key", false),
            ("# comment key abc", false),
        ];
        for line in lines {
            assert!(
                PublicSSHKey::maybe_from_string(line.0).is_ok() == line.1,
                "creation from '{}' must be '{}'",
                line.0,
                line.1
            );
        }
    }

    #[test]
    fn test_whitespace() {
        let lines = vec![
            ("ssh-key abc comment", "ssh-key abc comment"),
            (
                "ssh-key abc comment with spaces",
                "ssh-key abc comment with spaces",
            ),
            ("ssh-key abc", "ssh-key abc"),
            ("    ssh-key abc", "ssh-key abc"),
            ("ssh-key abc comment               ", "ssh-key abc comment"),
            ("ssh-key              abc comment", "ssh-key abc comment"),
            (
                "ssh-key              abc         comment",
                "ssh-key abc comment",
            ),
        ];
        for line in lines {
            assert!(
                PublicSSHKey::maybe_from_string(line.0).unwrap().to_string() == line.1,
                "key from '{}' must be '{}'",
                line.0,
                line.1
            );
        }
    }
}
