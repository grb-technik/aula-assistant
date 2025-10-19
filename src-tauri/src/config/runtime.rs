use std::path::PathBuf;

pub struct RuntimeConfigBuilder {
    tablet_mode: Option<bool>,
    config_file_path: Option<PathBuf>,
    fullscreen: Option<bool>,
}

impl RuntimeConfigBuilder {
    pub fn new() -> Self {
        RuntimeConfigBuilder {
            tablet_mode: None,
            config_file_path: None,
            fullscreen: None,
        }
    }

    pub fn tablet_mode(mut self, value: bool) -> Self {
        self.tablet_mode = Some(value);
        self
    }

    pub fn fullscreen(mut self, value: bool) -> Self {
        self.fullscreen = Some(value);
        self
    }

    pub fn config_file(mut self, path: Option<PathBuf>) -> Self {
        self.config_file_path = path;
        self
    }

    pub fn build(self) -> RuntimeConfig {
        RuntimeConfig::new(self.tablet_mode, self.fullscreen, self.config_file_path)
    }
}

pub struct RuntimeConfig {
    tablet_mode: Option<bool>,
    config_file_path: Option<PathBuf>,
    fullscreen: Option<bool>,
}

impl RuntimeConfig {
    pub fn new(
        tablet_mode: Option<bool>,
        fullscreen: Option<bool>,
        config_file_path: Option<PathBuf>,
    ) -> Self {
        RuntimeConfig {
            tablet_mode,
            fullscreen,
            config_file_path,
        }
    }

    pub fn tablet_mode(&self) -> Option<bool> {
        self.tablet_mode
    }

    pub fn fullscreen(&self) -> Option<bool> {
        self.fullscreen
    }

    pub fn config_file_path(&self) -> Option<&PathBuf> {
        self.config_file_path.as_ref()
    }
}
