use std::fs;
use std::path::Path;

use crate::Result;
use anyhow::anyhow;

pub fn write_keys_to_file(keys: &str, destination: &Path) -> Result<()> {
    if destination.is_dir() {
        return Err(anyhow!("destination must not be a directory"));
    }
    if destination.parent().is_some() {
        let parent = destination.parent().unwrap();
        if !parent.is_dir() {
            return Err(anyhow!("destination parent folder must exist"));
        }
    }
    fs::write(destination, keys).map_err(|e| anyhow::format_err!(e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_writing_to_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_path = temp_dir.path();
        let keys = "test";

        // cannot save to dir directly
        let mut dest = temp_path;
        assert!(write_keys_to_file(keys, dest).is_err());

        // cannot save to non-existing dir
        let mut dest_buf = temp_path.to_path_buf().join("idonotexist").join("keys");
        dest = dest_buf.as_path();
        assert!(write_keys_to_file(keys, dest).is_err());

        dest_buf = temp_path.to_path_buf().join("keys");
        dest = dest_buf.as_path();
        // existing dir
        assert!(write_keys_to_file(keys, dest).is_ok());
    }
}
