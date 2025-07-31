use crate::config::{Error, FileConfig};
use std::path::{Path, PathBuf};

pub(crate) fn parse_yaml(reader: impl std::io::Read) -> Result<FileConfig, Error> {
    serde_yaml::from_reader(reader).map_err(Error::YamlError)
}

pub(crate) fn open_config_file_as_reader(path: PathBuf) -> Result<impl std::io::Read, Error> {
    if path.exists() {
        std::fs::File::open(&path).map_err(Error::IoError)
    } else {
        Err(Error::FileNotFound(path))
    }
}

pub(crate) fn resolve_config_file_path(
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
