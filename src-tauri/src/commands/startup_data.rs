use crate::state::AppState;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

#[derive(Serialize, Deserialize)]
pub struct StartupData {
    build: BuildInfo,
    show_appbar: bool,
}

#[derive(Serialize, Deserialize)]
pub struct BuildInfo {
    commit: CommitInfo,
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

    let commit_date = std::env::var("LAST_COMMIT_DATE").unwrap_or_else(|_| "unknown".to_string());
    let commit_short_id = std::env::var("LAST_COMMIT_ID").unwrap_or_else(|_| "unknown".to_string());
    let commit_long_id =
        std::env::var("LAST_COMMIT_ID_LONG").unwrap_or_else(|_| "unknown".to_string());
    let build_timestamp_utc =
        std::env::var("BUILD_TIMESTAMP_UTC").unwrap_or_else(|_| "unknown".to_string());
    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_string());

    let commit_info = CommitInfo {
        date: commit_date,
        short_id: commit_short_id,
        long_id: commit_long_id,
    };

    let build_info = BuildInfo {
        commit: commit_info,
        date: build_timestamp_utc,
        version,
    };

    StartupData {
        build: build_info,
        show_appbar,
    }
}
