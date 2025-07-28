use crate::config::RuntimeConfig;

pub struct AppState {
    show_appbar: bool,
}

impl AppState {
    pub fn new(show_appbar: bool) -> Self {
        AppState { show_appbar }
    }

    pub fn show_appbar(&self) -> bool {
        self.show_appbar
    }
}

impl From<&RuntimeConfig> for AppState {
    fn from(config: &RuntimeConfig) -> Self {
        AppState::new(!config.tablet_mode())
    }
}
