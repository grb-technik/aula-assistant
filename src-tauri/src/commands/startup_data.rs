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

    let commit_date = std::env::var("LAST_COMMIT_DATE").ok();
    let commit_short_id = std::env::var("LAST_COMMIT_ID").ok();
    let commit_long_id = std::env::var("LAST_COMMIT_ID_LONG").ok();
    let build_timestamp_utc =
        std::env::var("BUILD_TIMESTAMP_UTC").expect("BUILD_TIMESTAMP_UTC not set");
    let version = std::env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION not set");

    let commit_info: Option<CommitInfo>;

    if commit_date.is_none() || commit_short_id.is_none() || commit_long_id.is_none() {
        commit_info = None;
    } else {
        commit_info = Some(CommitInfo {
            date: commit_date.unwrap(),
            short_id: commit_short_id.unwrap(),
            long_id: commit_long_id.unwrap(),
        });
    }

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
