use std::path::PathBuf;

use crate::config::RuntimeConfig;

pub struct RuntimeConfigBuilder {
    tablet_mode: bool,
    config_file_path: Option<String>,
}

impl RuntimeConfigBuilder {
    pub fn new() -> Self {
        RuntimeConfigBuilder {
            tablet_mode: false,
            config_file_path: None,
        }
    }

    /// default: false
    pub fn tablet_mode(mut self, value: bool) -> Self {
        self.tablet_mode = value;
        self
    }

    pub fn config_file(mut self, path: String) -> Self {
        self.config_file_path = Some(path);
        self
    }

    pub fn build(self) -> RuntimeConfig {
        RuntimeConfig::new(self.tablet_mode, self.config_file_path.map(PathBuf::from))
    }
}
