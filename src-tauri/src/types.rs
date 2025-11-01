use std::sync::Arc;

use tauri::Wry;
use tauri_plugin_store::Store as TauriStore;

pub type Store = Arc<TauriStore<Wry>>;

