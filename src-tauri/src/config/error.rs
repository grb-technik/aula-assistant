use std::{fmt::Display, path::PathBuf};

#[derive(PartialEq, Eq)]
pub enum ConfigErrorKind {
    YamlError,
    IoError,
    FileNotFound,
}

#[derive(Debug)]
pub enum ConfigError {
    YamlError(serde_yaml::Error),
    IoError(std::io::Error),
    FileNotFound(PathBuf),
}

impl ConfigError {
    pub fn kind(&self) -> ConfigErrorKind {
        match self {
            ConfigError::YamlError(_) => ConfigErrorKind::YamlError,
            ConfigError::IoError(_) => ConfigErrorKind::IoError,
            ConfigError::FileNotFound(_) => ConfigErrorKind::FileNotFound,
        }
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ConfigError::YamlError(e) => format!("YAML error: {}", e),
                ConfigError::IoError(e) => format!("IO error: {}", e),
                ConfigError::FileNotFound(path) => format!("File not found: {}", path.display()),
            }
        )
    }
}

impl std::error::Error for ConfigError {}
