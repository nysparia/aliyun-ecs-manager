use tauri_specta::collect_commands;

pub mod auth;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub fn commands_builder() -> tauri_specta::Builder {
    tauri_specta::Builder::<tauri::Wry>::new().commands(collect_commands![
        greet,
        auth::current_access_key_credential,
        auth::validate_access_key_credentials,
        auth::fulfill_access_key_credentials
    ])
}
