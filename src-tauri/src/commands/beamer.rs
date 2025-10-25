use crate::state::AppState;
use ptmahdbt42::{
    CMD_BEAMER_POF, CMD_BEAMER_PON, execute_request,
    request::{Body, Method, Request},
};
use std::{collections::HashMap, sync::Mutex};
use tauri::State;

#[tauri::command]
pub fn set_beamer_power_state(
    power: bool,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let app_state = state.lock().expect("failed to lock AppState");

    let body_str = if power {
        CMD_BEAMER_PON
    } else {
        CMD_BEAMER_POF
    };

    let request = Request::new(
        app_state.ptmahdbt42_host().to_string(),
        app_state.ptmahdbt42_port(),
        Method::POST,
        "/cgi-bin/MMX32_Keyvalue.cgi".to_string(),
        HashMap::new(),
        Some(Body::String(body_str.to_string())),
    );

    let _ = execute_request(&request).map_err(|e| {
        log::error!("Failed to set beamer power state: {}", e);
        "failed to set beamer power state".to_string()
    })?;

    Ok(())
}
