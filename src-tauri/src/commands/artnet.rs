use crate::state::AppState;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn get_all_artnet_scenes(state: State<'_, Mutex<AppState>>) -> Vec<String> {
    let app_state = state.lock().expect("failed to lock AppState");
    let scenes = app_state.lighting_scenes();
    scenes.iter().map(|(name, _)| name.clone()).collect()
}
