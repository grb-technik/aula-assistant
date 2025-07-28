use crate::config::{RuntimeConfig, Schema};

pub struct AppStateBuilder {
    show_appbar: Option<bool>,
    advanced_pin: Option<String>,
}

impl AppStateBuilder {
    pub fn new() -> Self {
        AppStateBuilder {
            show_appbar: None,
            advanced_pin: None,
        }
    }

    pub fn show_appbar(&mut self, show: bool) {
        self.show_appbar = Some(show);
    }

    pub fn advanced_pin(&mut self, pin: String) {
        self.advanced_pin = Some(pin);
    }

    pub fn build(self) -> AppState {
        AppState {
            show_appbar: self.show_appbar.expect("show_appbar must be set"),
            advanced_pin: self.advanced_pin.expect("advanced_pin must be set"),
        }
    }
}

pub trait TakeFrom<T> {
    fn take_from(&mut self, config: T);
}

impl TakeFrom<&RuntimeConfig> for AppStateBuilder {
    fn take_from(&mut self, config: &RuntimeConfig) {
        if let Some(tablet_mode) = config.tablet_mode() {
            self.show_appbar(!tablet_mode);
        }
    }
}

impl TakeFrom<&Schema> for AppStateBuilder {
    fn take_from(&mut self, config: &Schema) {
        if self.show_appbar.is_none() {
            self.show_appbar(!config.tablet_mode_default());
        }

        self.advanced_pin(config.advanced_pin().to_string());
    }
}

pub struct AppState {
    show_appbar: bool,
    advanced_pin: String,
}

impl AppState {
    pub fn show_appbar(&self) -> bool {
        self.show_appbar
    }

    pub fn advanced_pin(&self) -> &str {
        &self.advanced_pin
    }
}
