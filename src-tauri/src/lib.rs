use std::sync::Mutex;
use tauri::{Manager, State};
use tauri_plugin_log::{Target, TargetKind};

pub struct RuntimeConfigBuilder {
    tablet_mode: bool,
}

impl RuntimeConfigBuilder {
    pub fn new() -> Self {
        RuntimeConfigBuilder { tablet_mode: false }
    }

    /// default: false
    pub fn tablet_mode(mut self, value: bool) -> Self {
        self.tablet_mode = value;
        self
    }

    pub fn build(self) -> RuntimeConfig {
        RuntimeConfig::new(self.tablet_mode)
    }
}

pub struct RuntimeConfig {
    tablet_mode: bool,
}

impl RuntimeConfig {
    pub fn new(tablet_mode: bool) -> Self {
        RuntimeConfig { tablet_mode }
    }

    pub fn tablet_mode(&self) -> bool {
        self.tablet_mode
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
        AppState::new(!config.tablet_mode())
    }
}

pub fn run(config: RuntimeConfig) -> tauri::Result<()> {
    let app_state = AppState::from(config);

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .clear_targets()
                .target(Target::new(TargetKind::LogDir {
                    file_name: Some("logs".into()),
                }))
                .max_file_size(1024 * 1024 * 10) // 10 MB
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepOne)
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .build(),
        )
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
