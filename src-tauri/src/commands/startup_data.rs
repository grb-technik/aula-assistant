use crate::state::AppState;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

#[derive(Serialize, Deserialize)]
pub struct StartupData {
    build: BuildInfo,
    show_appbar: bool,
    open_in_fullscreen: bool,
}

#[derive(Serialize, Deserialize)]
pub struct BuildInfo {
    commit: Option<CommitInfo>,
    date: String,
    version: String,
}

#[derive(Serialize, Deserialize)]
pub struct CommitInfo {
    date: String,
    short_id: String,
    long_id: String,
}

#[tauri::command]
pub fn get_startup_data(state: State<'_, Mutex<AppState>>) -> StartupData {
    let state = state.lock().expect("failed to lock AppState");

    let show_appbar = state.show_appbar();
    let open_in_fullscreen = state.open_in_fullscreen();

    let commit_date = option_env!("LAST_COMMIT_DATE").take();
    let commit_short_id = option_env!("LAST_COMMIT_ID").take();
    let commit_long_id = option_env!("LAST_COMMIT_ID_LONG").take();
    let build_timestamp_utc =
        option_env!("BUILD_TIMESTAMP_UTC").expect("BUILD_TIMESTAMP_UTC not set");
    let version = option_env!("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION not set");

    let commit_info: Option<CommitInfo>;
    match (commit_date, commit_short_id, commit_long_id) {
        (Some(date), Some(short_id), Some(long_id)) => {
            commit_info = Some(CommitInfo {
                date: date.to_string(),
                short_id: short_id.to_string(),
                long_id: long_id.to_string(),
            });
        }
        _ => {
            commit_info = None;
        }
    }

    let build_info = BuildInfo {
        commit: commit_info,
        date: build_timestamp_utc.to_string(),
        version: version.to_string(),
    };

    StartupData {
        build: build_info,
        show_appbar,
        open_in_fullscreen,
    }
}
