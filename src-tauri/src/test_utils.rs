use tauri::{plugin::TauriPlugin, Runtime};

pub fn log_plugin_builder<R: Runtime>() -> TauriPlugin<R> {
    tauri_plugin_log::Builder::default()
        .target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::Stdout,
        ))
        .build()
}
