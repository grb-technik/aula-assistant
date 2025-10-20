use crate::{
    config::RuntimeConfig,
    state::{AppStateBuilder, TakeFrom},
};
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};

mod commands;
pub mod config;
mod state;

pub fn run(runtime_config: RuntimeConfig) -> tauri::Result<()> {
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
            let file_config: config::FileConfig =
                config::load_app_config(runtime_config.config_file_path(), app).map_err(|e| {
                    log::error!("failed to load app config: {}", e);
                    e
                })?;

            let mut app_state_builder = AppStateBuilder::new();
            // !!! do not change order !!!
            app_state_builder.take_from(&runtime_config);
            app_state_builder.take_from(&file_config);

            let state = app_state_builder.build();

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
