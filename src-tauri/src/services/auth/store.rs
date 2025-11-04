use serde::Serialize;

use crate::{
    services::auth::types::AccessKeyCredentials,
    types::{SerdeJsonError, Store},
};

pub trait AuthStore<C> {
    fn save(&self, credential: C) -> Result<(), SaveCredentialError>;
    fn query(&self) -> Result<C, QueryCredentialError>;
    fn delete(&self) -> bool;
}

#[derive(Debug, thiserror::Error, Serialize, specta::Type)]
#[serde(tag = "type", content = "error")]
pub enum QueryCredentialError {
    #[error("access key credential hasn't been saved")]
    NotExist,
    #[error("failed to deserialize credential using serde_json: {}", .0.0)]
    DeserializeError(#[from] SerdeJsonError),
}

#[derive(Debug, thiserror::Error, Serialize, specta::Type)]
pub enum SaveCredentialError {
    #[error("failed to serialize credential using serde_json: {}", 0.0)]
    SerializeError(#[from] SerdeJsonError),
}

impl From<serde_json::Error> for QueryCredentialError {
    fn from(value: serde_json::Error) -> Self {
        let error: SerdeJsonError = value.into();
        error.into()
    }
}

impl From<serde_json::Error> for SaveCredentialError {
    fn from(value: serde_json::Error) -> Self {
        let error: SerdeJsonError = value.into();
        error.into()
    }
}

pub struct AccessKeyAuthStore<R: tauri::Runtime> {
    store: Store<R>,
}

const ACCESS_KEY_CREDENTIAL_STORE_KEY: &str = "access_key_credential";

impl<R: tauri::Runtime> AccessKeyAuthStore<R> {
    pub fn new(store: Store<R>) -> Self {
        Self { store }
    }
}

impl<R: tauri::Runtime> AuthStore<AccessKeyCredentials> for AccessKeyAuthStore<R> {
    fn save(&self, credential: AccessKeyCredentials) -> Result<(), SaveCredentialError> {
        let to_valued = serde_json::to_value(credential)?;
        self.store.set(ACCESS_KEY_CREDENTIAL_STORE_KEY, to_valued);
        Ok(())
    }

    fn query(&self) -> Result<AccessKeyCredentials, QueryCredentialError> {
        self.store
            .get(ACCESS_KEY_CREDENTIAL_STORE_KEY)
            .ok_or(QueryCredentialError::NotExist)
            .and_then(|value| {
                let result = serde_json::from_value::<AccessKeyCredentials>(value);
                result.map_err(QueryCredentialError::from)
            })
    }

    fn delete(&self) -> bool {
        self.store.delete(ACCESS_KEY_CREDENTIAL_STORE_KEY)
    }
}

impl<R: tauri::Runtime> From<Store<R>> for AccessKeyAuthStore<R> {
    fn from(value: Store<R>) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
pub mod store_test_utils {
    use std::path::PathBuf;

    use tauri::test::{mock_builder, MockRuntime};
    use tauri_plugin_store::StoreBuilder;
    use tempfile::TempDir;

    use super::*;

    pub fn auth_store_path() -> PathBuf {
        let temp_dir = TempDir::new().unwrap();
        let store_path = temp_dir.path().join("test_auth_store.json");
        store_path
    }

    pub fn init_auth_store() -> AccessKeyAuthStore<MockRuntime> {
        let store_path = auth_store_path();

        let app = mock_builder()
            .invoke_handler(tauri::generate_handler![])
            .plugin(tauri_plugin_store::Builder::new().build())
            .build(tauri::generate_context!())
            .unwrap();

        let store = StoreBuilder::new(&app, store_path).build().unwrap();
        AccessKeyAuthStore::new(store)
    }
}

#[cfg(test)]
mod tests {
    use claims::assert_matches;
    use std::fmt::Debug;

    use crate::services::auth::types::AccessKeyCredentials;

    use super::*;

    #[test]
    fn test_access_key_auth_store() {
        let auth_store = store_test_utils::init_auth_store();

        let result = auth_store.query();
        assert_not_exist(result);

        let valid_credentials = vec![
            AccessKeyCredentials::new("", ""),
            AccessKeyCredentials::new("YOUR_ACCESS_KEY_ID", "YOUR_ACCESS_KEY_SECRET"),
        ];

        for credential in valid_credentials {
            auth_store.save(credential.clone()).unwrap();
            let queried_credential = auth_store
                .query()
                .map_err(|err| {
                    unreachable!(
                        "Failed to save the credential {:?}, error: {:?}",
                        credential, err
                    )
                })
                .unwrap();
            assert_eq!(credential.access_key_id, queried_credential.access_key_id);
            assert_eq!(
                credential.access_key_secret,
                queried_credential.access_key_secret
            );
        }

        let deleted = auth_store.delete();
        assert_eq!(deleted, true);
        let result = auth_store.query();
        assert_not_exist(result);
    }

    fn assert_not_exist<T: Debug>(result: Result<T, QueryCredentialError>) {
        assert_matches!(
            result
                .map(|value| { unreachable!("The store is not empty, value: {:?}", value) })
                .unwrap_err(),
            QueryCredentialError::NotExist
        );
    }
}
