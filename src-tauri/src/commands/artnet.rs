use crate::state::AppState;
use artnet::{build_artnet_package, send_artnet_package};
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn get_all_artnet_scenes(state: State<'_, Mutex<AppState>>) -> Vec<String> {
    let app_state = state.lock().expect("failed to lock AppState");
    let scenes = app_state.lighting_scenes();
    scenes.iter().map(|(name, _)| name.clone()).collect()
}

#[tauri::command]
pub fn run_artnet_scene(
    scene_name: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let app_state = state.lock().expect("failed to lock AppState");
    let scenes = app_state.lighting_scenes();
    let scene = scenes
        .get(&scene_name)
        .ok_or("scene not found".to_string())?;
    let universe = app_state.artnet_universe();
    let addr = &app_state.artnet_target();
    let socket = app_state.artnet_socket();
    {
        let mut artnet_data_mut = app_state.artnet_data();

        for (i, value) in scene.channels().iter().enumerate() {
            if let Some(v) = value {
                artnet_data_mut[i] = *v;
            }
        }

        let package = build_artnet_package(&universe, &*artnet_data_mut);
        send_artnet_package(&socket, *addr, &package).expect("failed to send artnet package");
    }
    Ok(())
}
