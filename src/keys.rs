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

pub fn concatenate_key_strings(strs: Vec<String>) -> String {
    let mut buf = String::new();
    for s in strs {
        buf.push_str(&format!("{}\n", s));
    }
    buf
}

pub fn keys_to_string(source: &str, keys: Vec<PublicSSHKey>) -> Result<String> {
    if keys.is_empty() {
        return Err(anyhow!("no valid keys from '{}'", source));
    }
    let keys_string = keys
        .iter()
        .fold(String::new(), |s, l| s + l.to_string().as_str() + "\n");
    Ok(format!("# keys from {}\n{}", source, keys_string))
}

pub fn url_to_keys(url: &Url, timeout: &Duration) -> Result<Vec<PublicSSHKey>> {
    let url_contents = url_contents_to_string(url, timeout)?;
    Ok(url_contents
        .lines()
        .flat_map(PublicSSHKey::maybe_from_string)
        .collect())
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
            ("ssh-rsa abc comment", true),
            ("ssh-rsa abc comment test", true),
            ("ssh-rsa abc", true),
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
    fn test_key_whitespace() {
        let lines = vec![
            ("ssh-rsa abc comment", "ssh-rsa abc comment"),
            (
                "ssh-rsa abc comment with spaces",
                "ssh-rsa abc comment with spaces",
            ),
            ("ssh-rsa abc", "ssh-rsa abc"),
            ("    ssh-rsa abc", "ssh-rsa abc"),
            ("ssh-rsa abc comment               ", "ssh-rsa abc comment"),
            ("ssh-rsa              abc comment", "ssh-rsa abc comment"),
            (
                "ssh-rsa              abc         comment",
                "ssh-rsa abc comment",
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

    #[test]
    fn test_keys_to_string() {
        let keys = vec![PublicSSHKey::maybe_from_string("ssh-rsa abc comment").unwrap()];
        let source = "https://example.com";
        let res = keys_to_string(source, keys).unwrap();
        // one after source, one after key
        assert!(res.lines().count() == 2);
    }

    #[test]
    fn test_empty_keys_to_string() {
        let keys = vec![];
        let source = "https://example.com";
        assert!(keys_to_string(source, keys).is_err());
    }

    #[test]
    fn test_concatenate_keys_to_string() {
        let strings = vec!["# test\ntest\n".to_owned()];
        let concat = concatenate_key_strings(strings);
        // adds additional line at the end to separate groups of keys
        assert!(concat.lines().count() == 3);
    }
}
