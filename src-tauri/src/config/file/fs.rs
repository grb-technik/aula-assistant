use std::{fs, path::PathBuf};

use crate::config::ConfigError;

pub fn read_config_file(path: PathBuf) -> Result<Option<String>, ConfigError> {
    if path.exists() {
        match fs::read_to_string(&path) {
            Ok(content) => {
                if content.is_empty() {
                    return Ok(None);
                }
                return Ok(Some(content));
            }
            Err(e) => return Err(ConfigError::IoError(e)),
        }
    } else {
        Err(ConfigError::FileNotFound(path))
    }
}
