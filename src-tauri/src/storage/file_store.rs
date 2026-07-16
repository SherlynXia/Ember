use std::fs;
use std::path::PathBuf;
use serde_json;
use crate::models::AppState;

const STATE_FILE: &str = "ember_data.json";

pub fn get_state_path() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("ember_app");
    fs::create_dir_all(&path).ok();
    path.push(STATE_FILE);
    path
}

pub fn load_state() -> AppState {
    let path = get_state_path();
    if let Ok(content) = fs::read_to_string(&path) {
        serde_json::from_str(&content).unwrap_or_else(|_| AppState::default())
    } else {
        AppState::default()
    }
}

pub fn save_state(state: &AppState) -> Result<(), String> {
    let path = get_state_path();
    let content = serde_json::to_string_pretty(state).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())
}