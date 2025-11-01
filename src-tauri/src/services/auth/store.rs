use serde::Serialize;

use crate::{
    services::auth::types::AccessKeyCredential,
    types::{SerdeJsonError, Store},
};

pub trait AuthStore<C> {
    fn save(&self, credential: C);
    fn query(&self) -> anyhow::Result<C>;
    fn remove(&self) -> ();
}

#[derive(Debug, thiserror::Error, Serialize, specta::Type)]
pub enum QueryError {
    #[error("access key credential hasn't been saved")]
    NotExist,
    #[error("failed to deserialize using serde_json: {}", .0.0)]
    DeserializeError(#[from] SerdeJsonError),
}

impl From<serde_json::Error> for QueryError {
    fn from(value: serde_json::Error) -> Self {
        let error: SerdeJsonError = value.into();
        error.into()
    }
}

pub struct AccessKeyAuthStore {
    store: Store,
}

const ACCESS_KEY_CREDENTIAL_STORE_KEY: &str = "access_key_credential";

impl AccessKeyAuthStore {
    pub fn new(store: Store) -> Self {
        Self { store }
    }
    pub fn save(&self) {
        todo!()
    }

    pub fn query(&self) -> Result<AccessKeyCredential, QueryError> {
        self.store
            .get(ACCESS_KEY_CREDENTIAL_STORE_KEY)
            .ok_or(QueryError::NotExist)
            .and_then(|value| {
                let result = serde_json::from_value::<AccessKeyCredential>(value);
                result.map_err(QueryError::from)
            })
    }
}
