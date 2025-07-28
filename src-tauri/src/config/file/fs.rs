use std::{fs, path::PathBuf};

use crate::config::ConfigError;

pub fn read_config_file(config_dir: &PathBuf) -> Result<Option<String>, ConfigError> {
    let config_path = config_dir.join("config.yaml");

    if config_path.exists() {
        match fs::read_to_string(&config_path) {
            Ok(content) => {
                if content.is_empty() {
                    return Ok(None);
                }
                return Ok(Some(content));
            }
            Err(e) => return Err(ConfigError::IoError(e)),
        }
    }

    Ok(None)
}
