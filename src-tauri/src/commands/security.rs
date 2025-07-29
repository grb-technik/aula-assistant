use crate::state::AppState;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn check_advanced_pin(state: State<'_, Mutex<AppState>>, pin: String) -> bool {
    let state = state.lock().expect("failed to lock AppState");
    state.advanced_pin() == pin
}
