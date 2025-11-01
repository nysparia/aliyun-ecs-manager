use tauri::Manager;
use tauri_plugin_store::StoreBuilder;
use tauri_specta::collect_commands;

use crate::services::auth::AccessKeyAuthService;

pub mod commands;
pub mod services;
pub mod types;
pub mod test_utils;
pub mod init;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let commands_builder = tauri_specta::Builder::<tauri::Wry>::new().commands(collect_commands![
        greet,
        commands::auth::current_access_key_credential
    ]);

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(commands_builder.invoke_handler())
        .setup(|app| {
            let builder = StoreBuilder::new(app, "store.json");
            let store = builder.build().expect("Store plugin build failed");
            app.manage(AccessKeyAuthService::new(store));
            Result::Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
