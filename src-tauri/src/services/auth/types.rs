use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, specta::Type, Clone, PartialEq)]
pub struct AccessKeyCredentials {
    pub access_key_id: String,
    pub access_key_secret: String,
}

impl AccessKeyCredentials {
    pub fn new<T1: Into<String>, T2: Into<String>>(
        access_key_id: T1,
        access_key_secret: T2,
    ) -> Self {
        let access_key_id = access_key_id.into();
        let access_key_secret = access_key_secret.into();
        Self {
            access_key_id,
            access_key_secret,
        }
    }
}
