// use tauri::Manager;
// use tauri_plugin_store::StoreBuilder;
// use tauri_specta::collect_commands;

// use crate::services::auth::AccessKeyAuthService;

// pub fn create_tauri_app<R: tauri::Runtime, F>(
//     builder: tauri::Builder<R>,
//     commands_builder: Option<tauri_specta::Builder<R>>,
//     setup: F,
// ) -> tauri::App<R>
// where
//     F: FnOnce(&mut tauri::App<R>) -> std::result::Result<(), Box<dyn std::error::Error>> + Send,
// {
//     let builder = builder
//         .plugin(tauri_plugin_store::Builder::new().build())
//         .plugin(tauri_plugin_opener::init());

//     let builder = if let Some(value) = commands_builder {
//         builder.invoke_handler(value.invoke_handler())
//     } else {
//         builder.invoke_handler(
//             tauri_specta::Builder::<R>::new()
//                 .commands(collect_commands![])
//                 .invoke_handler(),
//         )
//     };

//     builder.setup(setup).build(tauri::generate_context!())
// }
