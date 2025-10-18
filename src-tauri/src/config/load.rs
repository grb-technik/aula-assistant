use tauri::Manager;

use crate::config::{self, RuntimeConfig};

pub(crate) fn load_app_config(
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
