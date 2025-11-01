use serde::Serialize;

#[derive(Debug, Serialize, specta::Type)]
pub struct AccessKeyCredential {
    access_key_id: String,
    access_key_secret: String,
}

#[derive(thiserror::Error, Debug, Serialize, specta::Type)]
#[error("{description}")]
pub struct BoxedError {
    description: String,
    #[serde(skip)]
    source: anyhow::Error,
}
