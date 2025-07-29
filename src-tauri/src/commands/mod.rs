use crate::state::AppState;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn get_show_appbar(state: State<'_, Mutex<AppState>>) -> bool {
    let state = state.lock().expect("failed to lock AppState");
    state.show_appbar()
}

#[tauri::command]
pub fn check_advanced_pin(state: State<'_, Mutex<AppState>>, pin: String) -> bool {
    let state = state.lock().expect("failed to lock AppState");
    state.advanced_pin() == pin
}

#[tauri::command]
pub fn get_build_info() -> [String; 5] {
    let commit_date = std::env::var("LAST_COMMIT_DATE").unwrap_or_else(|_| "unknown".to_string());
    let commit_short_id = std::env::var("LAST_COMMIT_ID").unwrap_or_else(|_| "unknown".to_string());
    let commit_long_id =
        std::env::var("LAST_COMMIT_ID_LONG").unwrap_or_else(|_| "unknown".to_string());
    let build_timestamp_utc =
        std::env::var("BUILD_TIMESTAMP_UTC").unwrap_or_else(|_| "unknown".to_string());
    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_string());

    return [
        commit_date,
        commit_short_id,
        commit_long_id,
        build_timestamp_utc,
        version,
    ];
}
