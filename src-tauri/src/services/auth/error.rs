use alibabacloud::client::error::{AdvancedClientError, AliyunRejection};
use serde::Serialize;
use thiserror::Error;

use crate::services::auth::store::SaveCredentialError;

#[derive(specta::Type)]
#[allow(dead_code)]
struct AliyunRejectionTypeShadow {
    pub code: String,
    pub host_id: String,
    pub message: String,
    pub request_id: String,
    pub recommend: String,
}

#[derive(specta::Type)]
#[allow(dead_code)]
enum AdvanceClientErrorTypeShadow {
    AliyunRejectError(AliyunRejectionTypeShadow),
    UnderlyingError(String),
    ResultDeserializationError(String),
}

#[derive(Error, Debug, Serialize, specta::Type)]
pub enum AKValidationError {
    #[error("the access key credentials provided is not valid: {}", .0.code)]
    NotValid(#[specta(type = AliyunRejectionTypeShadow)] AliyunRejection),
    #[error("when using the sts aliyun client to send request it throw an error: {0}")]
    UnderlyingError(
        #[from]
        #[specta(type = AdvanceClientErrorTypeShadow)]
        AdvancedClientError,
    ),
}

#[derive(Error, Debug, Serialize, specta::Type)]
pub enum AKFulfillError {
    #[error("the access key credentials provided is not valid: {}", .0.code)]
    NotValid(#[specta(type = AliyunRejectionTypeShadow)] AliyunRejection),
    #[error("failed to save credential into the auth store")]
    SaveCredentialError(#[from] SaveCredentialError),
    #[error("when using the sts aliyun client to send request it throw an error: {0}")]
    UnderlyingError(
        #[from]
        #[specta(type = AdvanceClientErrorTypeShadow)]
        AdvancedClientError,
    ),
}

impl AKValidationError {
    pub fn new_underlying(rejection: AliyunRejection) -> Self {
        Self::UnderlyingError(AdvancedClientError::AliyunRejectError(rejection))
    }
}

impl From<AKValidationError> for AKFulfillError {
    fn from(value: AKValidationError) -> Self {
        match value {
            AKValidationError::NotValid(aliyun_rejection) => {
                AKFulfillError::NotValid(aliyun_rejection)
            }
            AKValidationError::UnderlyingError(advanced_client_error) => {
                AKFulfillError::UnderlyingError(advanced_client_error)
            }
        }
    }
}
