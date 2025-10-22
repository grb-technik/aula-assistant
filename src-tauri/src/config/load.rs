use tauri::Manager;

use crate::config::{Error, FileConfig, Validate};
use std::path::{Path, PathBuf};

pub fn load_app_config(
    args_config_path: Option<&PathBuf>,
    app: &tauri::App,
) -> Result<FileConfig, Error> {
    let config_path = resolve_config_file_path(app.path().app_config_dir(), args_config_path)?;

    let reader = match open_config_file_as_reader(&config_path) {
        Ok(reader) => {
            log::info!("using config path: {}", config_path.display());
            Some(reader)
        }
        Err(e) => match e {
            Error::FileNotFound(_) => {
                log::info!("config file not found, using default settings.");
                None
            }
            _ => {
                log::error!("failed to open config file: {}", e);
                return Err(e);
            }
        },
    };

    if reader.is_none() {
        return Ok(FileConfig::default());
    }

    let config = match parse_yaml(reader.unwrap()) {
        Ok(parsed) => Ok(parsed),
        Err(e) => {
            log::info!("failed to parse config file: {}", e);
            Err(e)
        }
    }?;

    if let Err(e) = config.validate() {
        log::error!("config validation error: {}", e);
        return Err(Error::FileValidationError(e));
    }

    Ok(config)
}

fn parse_yaml(reader: impl std::io::Read) -> Result<FileConfig, Error> {
    serde_yaml::from_reader(reader).map_err(Error::YamlError)
}

fn open_config_file_as_reader(path: &PathBuf) -> Result<std::fs::File, Error> {
    if path.exists() {
        std::fs::File::open(path).map_err(Error::IoError)
    } else {
        Err(Error::FileNotFound(path.to_path_buf()))
    }
}

fn resolve_config_file_path(
    app_config_dir: tauri::Result<PathBuf>,
    args_config_path: Option<&PathBuf>,
) -> Result<PathBuf, Error> {
    let current_dir = std::env::current_dir().map_err(Error::IoError)?;

    let base_path = match args_config_path {
        Some(path) if path == Path::new(".") => current_dir.clone(),
        Some(path) => path.clone(),
        None => app_config_dir.map_err(Error::AppDirNotFound)?,
    };

    let absolute_path = if base_path.is_relative() {
        current_dir.join(base_path)
    } else {
        base_path
    };

    Ok(if absolute_path.ends_with("config.yaml") {
        absolute_path
    } else {
        absolute_path.join("config.yaml")
    })
}
