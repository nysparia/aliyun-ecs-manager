use alibabacloud::client::error::{AdvancedClientError, AliyunRejection};
use thiserror::Error;

use crate::services::auth::store::SaveCredentialError;

#[derive(Error, Debug)]
pub enum AKFulfillError {
    #[error("the access key credentials provided is not valid: {}", .0.code)]
    NotValid(AliyunRejection),
    #[error("failed to save credential into the auth store")]
    SaveCredentialError(#[from] SaveCredentialError),
    #[error("when using the sts aliyun client to send request it throw an error: {0}")]
    UnderlyingError(#[from] AdvancedClientError),
}

impl AKFulfillError {
    pub fn new_underlying(rejection: AliyunRejection) -> Self {
        Self::UnderlyingError(AdvancedClientError::AliyunRejectError(rejection))
    }
}
