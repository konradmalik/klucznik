use std::fs;
use std::path::Path;

use crate::Result;
use anyhow::anyhow;

pub fn write_keys_to_file(keys: &str, destination: &Path) -> Result<()> {
    if destination.is_dir() {
        return Err(anyhow!("destination must not be a directory"));
    }
    if destination.is_file() {
        // need to check for modifications
        let old_digest = sha256::try_digest(destination)?;
        let new_digest = sha256::digest(keys);
        if old_digest == new_digest {
            eprintln!("not overwriting the destination, keys have not changed");
            return Ok(());
        }
    }
    fs::write(destination, keys).map_err(|e| anyhow::format_err!(e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_writing_to_dir() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_path = temp_dir.path();
        let keys = "test";

        // cannot save to dir directly
        let dest = temp_path;
        assert!(write_keys_to_file(keys, dest).is_err());

        temp_dir.close().expect("problems removing temp dir");
    }

    #[test]
    fn test_writing_to_non_existing_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_path = temp_dir.path();
        let keys = "test";

        // cannot save to non-existing dir
        let dest_buf = temp_path.to_path_buf().join("idonotexist").join("keys");
        let dest = dest_buf.as_path();
        assert!(
            write_keys_to_file(keys, dest).is_err(),
            "should not write to '{}'",
            dest_buf.to_string_lossy()
        );

        temp_dir.close().expect("problems removing temp dir");
    }

    #[test]
    fn test_writing_to_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_path = temp_dir.path();
        let keys = "test";

        // existing dir
        let dest_buf = temp_path.join("keys");
        let wrote = write_keys_to_file(keys, &dest_buf);
        assert!(
            wrote.is_ok(),
            "failed writing to: {} because: {}",
            dest_buf.to_string_lossy(),
            wrote.unwrap_err()
        );

        // get stat of the file
        let old_metadata = fs::metadata(&dest_buf).unwrap();
        write_keys_to_file(keys, &dest_buf).unwrap();
        let new_metadata = fs::metadata(&dest_buf).unwrap();
        assert!(old_metadata.modified().unwrap() == new_metadata.modified().unwrap());

        temp_dir.close().expect("problems removing temp dir");
    }
}
