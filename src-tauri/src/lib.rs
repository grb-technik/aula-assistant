use crate::config::RuntimeConfig;
use std::sync::Mutex;
use tauri::{Manager, State};
use tauri_plugin_log::{Target, TargetKind};

pub mod config;

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

impl From<&RuntimeConfig> for AppState {
    fn from(config: &RuntimeConfig) -> Self {
        AppState::new(!config.tablet_mode())
    }
}

pub fn run(config: RuntimeConfig) -> tauri::Result<()> {
    let app_state = AppState::from(&config);

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
        .setup(move |app| {
            let config_dir;

            if config.config_file_path().is_some() {
                config_dir = config
                    .config_file_path()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .to_path_buf();
            } else {
                config_dir = match app.path().app_config_dir() {
                    Ok(path) => path,
                    Err(e) => {
                        log::error!("failed to get app config directory: {}", e);
                        return Err(e.into());
                    }
                };
            }

            let config_file_content = config::read_config_file(&config_dir).map_err(|e| {
                log::error!("failed to read config file: {}", e);
                e
            });

            if let Some(content) = config_file_content.unwrap() {
                let _config_file_contents = config::yaml::parse_yaml(&content).map_err(|e| {
                    log::error!("failed to parse YAML config: {}", e);
                    e
                });
                // TODO: do smth with it
            }

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
