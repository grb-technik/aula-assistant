use crate::{
    config::RuntimeConfig,
    state::{AppStateBuilder, TakeFrom},
};
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};

mod artnet;
mod commands;
pub mod config;
mod state;

#[cfg(test)]
mod test;

pub fn run(config: RuntimeConfig) -> tauri::Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let webview_window = app.get_webview_window("main");

            match webview_window {
                Some(window) => {
                    if window.set_focus().is_err() {
                        log::error!("failed to set focus on the webview window.");
                    }
                }
                None => {
                    log::warn!("no webview window found with the label 'main'.");
                }
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
            let cfg = load_app_config(&config, app).map_err(|e| {
                log::error!("failed to load app config: {}", e);
                e
            })?;

            cfg.validate().map_err(|e| {
                log::error!("config validation failed: {:?}", e);
                e
            })?;

            let mut apb = AppStateBuilder::new();
            // !!! do not change order !!!
            apb.take_from(&config);
            apb.take_from(&cfg);

            let state = apb.build();

            app.manage(Mutex::new(state));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::startup_data::get_startup_data,
            commands::security::check_advanced_pin,
            commands::artnet::get_all_artnet_scenes,
            commands::artnet::run_artnet_scene
        ])
        .run(tauri::generate_context!())
}

fn load_app_config(
    config: &RuntimeConfig,
    app: &tauri::App,
) -> Result<config::FileConfig, config::Error> {
    let config_path =
        config::resolve_config_file_path(app.path().app_config_dir(), config.config_file_path())?;

    log::info!("using config path: {}", config_path.display());

    let reader = match config::open_config_file_as_reader(config_path) {
        Ok(reader) => Some(reader),
        Err(e) => match e {
            config::Error::FileNotFound(_) => {
                log::info!("config file not found, using default settings.");
                None
            }
            _ => {
                log::error!("failed to open config file: {}", e);
                return Err(e);
            }
        },
    };

    if reader.is_none() {
        return Ok(config::FileConfig::default());
    }

    match config::parse_yaml(reader.unwrap()) {
        Ok(parsed) => Ok(parsed),
        Err(e) => {
            log::info!("failed to parse config file: {}", e);
            Err(e)
        }
    }
}
