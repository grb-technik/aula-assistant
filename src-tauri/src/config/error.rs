use std::{fmt::Display, path::PathBuf};

#[derive(Debug)]
pub enum Error {
    YamlError(serde_yaml::Error),
    IoError(std::io::Error),
    FileNotFound(PathBuf),
    AppDirNotFound(tauri::Error),
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Error::YamlError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<tauri::Error> for Error {
    fn from(err: tauri::Error) -> Self {
        Error::AppDirNotFound(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::YamlError(e) => format!("yaml error: {}", e),
                Error::IoError(e) => format!("io error: {}", e),
                Error::FileNotFound(path) => format!("file not found: {}", path.display()),
                Error::AppDirNotFound(e) => format!("app directory not found: {}", e),
            }
        )
    }
}

impl std::error::Error for Error {}
