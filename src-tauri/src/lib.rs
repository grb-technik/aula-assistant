use std::sync::Mutex;
use tauri::{Manager, State};

pub struct RuntimeConfigBuilder {
    hide_appbar: bool,
}

impl RuntimeConfigBuilder {
    pub fn new() -> Self {
        RuntimeConfigBuilder { hide_appbar: false }
    }

    /// default: false
    pub fn hide_appbar(mut self, value: bool) -> Self {
        self.hide_appbar = value;
        self
    }

    pub fn build(self) -> RuntimeConfig {
        RuntimeConfig::new(self.hide_appbar)
    }
}

pub struct RuntimeConfig {
    hide_appbar: bool,
}

impl RuntimeConfig {
    pub fn new(hide_appbar: bool) -> Self {
        RuntimeConfig { hide_appbar }
    }

    pub fn hide_appbar(&self) -> bool {
        self.hide_appbar
    }
}

struct AppState {
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

impl From<RuntimeConfig> for AppState {
    fn from(config: RuntimeConfig) -> Self {
        AppState::new(!config.hide_appbar())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(config: RuntimeConfig) -> tauri::Result<()> {
    let app_state = AppState::from(config);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(Mutex::new(app_state));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_show_appbar])
        .run(tauri::generate_context!())
}

#[tauri::command]
fn get_show_appbar(state: State<'_, Mutex<AppState>>) -> bool {
    let state = state.lock().expect("failed to lock AppState");
    state.show_appbar()
}
