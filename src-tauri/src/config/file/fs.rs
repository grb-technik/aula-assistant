use std::path::PathBuf;

use crate::config::{Error, FileConfig};

pub(crate) fn parse_yaml(yaml: &str) -> Result<FileConfig, Error> {
    match serde_yaml::from_str(yaml) {
        Ok(parsed) => return Ok(parsed),
        Err(e) => return Err(Error::YamlError(e)),
    }
}

pub(crate) fn read_config_file(path: PathBuf) -> Result<Option<String>, Error> {
    if path.exists() {
        match std::fs::read_to_string(&path) {
            Ok(content) => {
                if content.is_empty() {
                    return Ok(None);
                }
                return Ok(Some(content));
            }
            Err(e) => return Err(Error::IoError(e)),
        }
    } else {
        Err(Error::FileNotFound(path))
    }
}

pub(crate) fn resolve_config_file_path(
    app_config_dir: tauri::Result<PathBuf>,
    args_config_path: Option<&PathBuf>,
) -> Result<PathBuf, Error> {
    let mut config_path;

    match args_config_path {
        Some(path) => {
            config_path = path.to_path_buf();
        }
        None => {
            config_path = app_config_dir.map_err(|e| Error::AppDirNotFound(e))?;
        }
    }

    let current_dir = std::env::current_dir().map_err(|e| Error::IoError(e))?;

    if config_path == PathBuf::from(".") {
        config_path = current_dir.clone();
    }

    if config_path.is_relative() {
        config_path = current_dir.join(config_path);
    }

    if !config_path.ends_with("config.yaml") {
        config_path = config_path.join("config.yaml");
    }

    Ok(config_path)
}
