use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, specta::Type)]
pub struct AccessKeyCredential {
    access_key_id: String,
    access_key_secret: String,
}