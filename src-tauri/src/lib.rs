use crate::{
    config::RuntimeConfig,
    state::{AppStateBuilder, TakeFrom},
};
use std::{path::PathBuf, sync::Mutex};
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};

mod commands;
pub mod config;
mod state;

pub fn run(config: RuntimeConfig) -> tauri::Result<()> {
    let mut apb = AppStateBuilder::new();
    apb.take_from(&config);

    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let webview_window = app.get_webview_window("main");

            if webview_window.is_none() {
                log::error!("no webview window found with the label 'main'.");
                return;
            }

            if webview_window.unwrap().set_focus().is_err() {
                log::error!("failed to set focus on the webview window.");
            }
        }))
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

            let config_file_content = config::read_config_file(config_path);

            let cfg;

            if let Err(e) = config_file_content {
                if e.kind() == config::ConfigErrorKind::FileNotFound {
                    log::info!("empty config file found, using default settings.");
                    cfg = config::Schema::default();
                } else {
                    log::error!("failed to read config file: {}", e);
                    return Err(e.into());
                }
            } else if let Some(content) = config_file_content.unwrap() {
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

            apb.take_from(&cfg);

            app.manage(Mutex::new(apb.build()));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_show_appbar,
            commands::check_advanced_pin,
            commands::get_build_info,
        ])
        .run(tauri::generate_context!())
}
