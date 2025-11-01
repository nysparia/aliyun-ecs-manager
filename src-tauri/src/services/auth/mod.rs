use crate::{services::auth::{
    store::{AccessKeyAuthStore, QueryError},
    types::AccessKeyCredential,
}, types::Store};

pub mod store;
pub mod types;

pub struct AccessKeyAuthService {
    auth_store: AccessKeyAuthStore,
}

impl AccessKeyAuthService {
    pub fn new(store: Store) -> Self {
        Self {
            auth_store: AccessKeyAuthStore::new(store)
        }
    }

    pub fn current_access_key_credential(&self) -> Result<AccessKeyCredential, QueryError> {
        self.auth_store.query()
    }
}
