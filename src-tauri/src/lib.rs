use specta_typescript::Typescript;
use tauri::Manager;
use tauri_plugin_store::StoreBuilder;

use crate::{
    commands::commands_builder,
    services::{auth::AccessKeyAuthService, client::AliyunClientService},
};

pub mod commands;
pub mod init;
pub mod services;
pub mod test_utils;
pub mod types;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let commands_builder = commands_builder();

    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .init();

    let log_builder = tauri_plugin_log::Builder::default()
        .target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::Stdout,
        ))
        .build();

    #[cfg(debug_assertions)]
    commands_builder
        .export(Typescript::default(), "../src/binding.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(log_builder)
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(commands_builder.invoke_handler())
        .setup(move |app| {
            commands_builder.mount_events(app);
            let builder = StoreBuilder::new(app, "store.json");
            let store = builder.build().expect("Store plugin build failed");
            let client_service = AliyunClientService::new();
            let auth_service = AccessKeyAuthService::new(store);
            
            if let Some(client) = auth_service.new_client() {
                log::info!("Successfully initialized Aliyun client from saved credentials");
                client_service.initialize(client);
            } else {
                log::info!("No valid credentials found, Aliyun client not initialized")
            }

            app.manage(auth_service);
            app.manage(client_service);

            Result::Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
