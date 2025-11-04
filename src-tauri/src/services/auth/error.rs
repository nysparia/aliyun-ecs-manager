use alibabacloud::client::error::AliyunRejection;
use serde::Serialize;
use thiserror::Error;

use crate::services::{
    auth::store::SaveCredentialError,
    error::{AliyunRejectionTypeShadow, AliyunRequestCommandError},
};

#[derive(specta::Type)]
#[allow(dead_code)]
enum AdvanceClientErrorTypeShadow {
    AliyunRejectError(AliyunRejectionTypeShadow),
    UnderlyingError(String),
    ResultDeserializationError(String),
}

#[derive(Debug, Error, Serialize, specta::Type)]
#[serde(transparent)]
#[error("the access key credentials provided is not valid: {}", .0.code)]
pub struct AKNotValid(#[specta(type = AliyunRejectionTypeShadow)] pub AliyunRejection);

impl From<SaveCredentialError> for AliyunRequestCommandError<AKNotValid> {
    fn from(value: SaveCredentialError) -> Self {
        AliyunRequestCommandError::InternalError {
            message: value.to_string(),
            source: Box::new(value),
        }
    }
}
