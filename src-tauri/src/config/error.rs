use std::{fmt::Display, path::PathBuf};

#[derive(Debug)]
pub enum Error {
    FileValidationError(FileValidationError),
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
                Error::FileValidationError(e) => format!("invalid config file: {}", e),
                Error::YamlError(e) => format!("yaml error: {}", e),
                Error::IoError(e) => format!("io error: {}", e),
                Error::FileNotFound(path) => format!("file not found: {}", path.display()),
                Error::AppDirNotFound(e) => format!("app directory not found: {}", e),
            }
        )
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub enum FileValidationError {
    SecurityAdvancedPinInvalid(String),
    ArtnetBindInvalid(String),
    ArtnetTargetInvalid(String),
    ArtnetUniverseInvalid(String),
    PatchedFixtureTypeInvalid(String),
    SceneSetInvalid(String),
}

impl std::fmt::Display for FileValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FileValidationError::SecurityAdvancedPinInvalid(str) => {
                    format!("security.advanced_pin is invalid: {}", str)
                }
                FileValidationError::ArtnetBindInvalid(str) => {
                    format!("lighting.artnet.bind is invalid: {}", str)
                }
                FileValidationError::ArtnetTargetInvalid(str) => {
                    format!("lighting.artnet.target is invalid: {}", str)
                }
                FileValidationError::ArtnetUniverseInvalid(str) => {
                    format!("lighting.artnet.universe is invalid: {}", str)
                }
                FileValidationError::PatchedFixtureTypeInvalid(str) => {
                    format!("lighting.patch.patched fixture type is invalid: {}", str)
                }
                FileValidationError::SceneSetInvalid(str) => {
                    format!("lighting.scenes scene set is invalid: {}", str)
                }
            }
        )
    }
}

impl std::error::Error for FileValidationError {}
