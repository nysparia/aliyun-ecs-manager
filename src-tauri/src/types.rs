use std::sync::Arc;

use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use tauri::Wry;
use tauri_plugin_store::Store as TauriStore;

pub type Store = Arc<TauriStore<Wry>>;

#[derive(thiserror::Error, Debug, Serialize, specta::Type)]
#[error("{description}")]
pub struct BoxedError {
    description: String,
    #[serde(skip)]
    source: anyhow::Error,
}

#[serde_as]
#[derive(thiserror::Error, Debug, Serialize, specta::Type)]
#[error("serde_json failed to de/serialize, {0}")]
pub struct SerdeJsonError(
    #[from]
    #[serde_as(as = "DisplayFromStr")]
    #[specta(type = String)]
    pub serde_json::Error,
);

impl From<anyhow::Error> for BoxedError {
    fn from(value: anyhow::Error) -> Self {
        Self {
            description: value.to_string(),
            source: value,
        }
    }
}
