use std::error::Error;

use serde::Serialize;
use serde_with::{serde_as, skip_serializing_none, DisplayFromStr};

use crate::{services::auth::types::{AccessKeyCredential, BoxedError}, types::Store};

pub trait AuthStore<C> {
    fn save(&self, credential: C);
    fn query(&self) -> anyhow::Result<C>;
    fn remove(&self) -> ();
}

#[derive(Debug, thiserror::Error, Serialize, specta::Type)]
pub enum QueryError {
    #[error("access key credential hasn't been saved")]
    NotExist,
    #[error("other errors happened when trying to query the access key credential")]
    OtherError(BoxedError),
}

pub struct AccessKeyAuthStore {
    store: Store,
}

impl AccessKeyAuthStore {
    pub fn new(store: Store) -> Self {
        Self { store }
    }
    pub fn save(&self) {
        todo!()
    }

    pub fn query(&self) -> Result<AccessKeyCredential, QueryError> {
        todo!()
    }
}
