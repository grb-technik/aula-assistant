use crate::{config::RuntimeConfig, state::AppState};
use std::{path::PathBuf, sync::Mutex};
use tauri::{Manager, State};
use tauri_plugin_log::{Target, TargetKind};

pub mod config;
pub mod state;

pub fn run(config: RuntimeConfig) -> tauri::Result<()> {
    let app_state = AppState::from(&config);

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .clear_targets()
                .target(Target::new(TargetKind::LogDir {
                    file_name: Some("logs".into()),
                }))
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Stdout,
                ))
                .max_file_size(1024 * 1024 * 10) // 10 MB
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepOne)
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            let mut config_path;

            if config.config_file_path().is_some() {
                config_path = config.config_file_path().unwrap().to_path_buf();
            } else {
                config_path = match app.path().app_config_dir() {
                    Ok(path) => path,
                    Err(e) => {
                        log::error!("failed to get app config directory: {}", e);
                        return Err(e.into());
                    }
                };
            }

            if config_path == PathBuf::from(".") {
                config_path = std::env::current_dir().map_err(|e| {
                    log::error!("failed to get current directory: {}", e);
                    e
                })?;
            }

            if config_path.is_relative() {
                config_path = std::env::current_dir()
                    .map_err(|e| {
                        log::error!("failed to get current directory: {}", e);
                        e
                    })?
                    .join(config_path);
            }

            if !config_path.ends_with("config.yaml") {
                config_path = config_path.join("config.yaml");
            }

            log::info!("using config path: {}", config_path.display());

            let config_file_content = config::read_config_file(config_path).map_err(|e| {
                log::error!("failed to read config file: {}", e);
                e
            });

            let cfg;

            if let Some(content) = config_file_content.unwrap() {
                cfg = config::yaml::parse_yaml(&content)
                    .map_err(|e| {
                        log::error!("failed to parse YAML config: {}", e);
                        e
                    })
                    .unwrap();
            } else {
                log::info!("empty config file found, using default settings.");
                cfg = config::Schema::default();
            }

            // TODO: do smth with the config
            log::debug!("parsed config: {}", cfg);

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
