use std::path::PathBuf;

pub struct RuntimeConfig {
    tablet_mode: Option<bool>,
    config_file_path: Option<PathBuf>,
}

impl RuntimeConfig {
    pub fn new(tablet_mode: Option<bool>, config_file_path: Option<PathBuf>) -> Self {
        RuntimeConfig {
            tablet_mode,
            config_file_path,
        }
    }

    pub fn tablet_mode(&self) -> Option<bool> {
        self.tablet_mode
    }

    pub fn config_file_path(&self) -> Option<&PathBuf> {
        self.config_file_path.as_ref()
    }
}
