use alibabacloud::client::error::{AliyunRejection, OperationError, RequestErrorKind};
use serde::Serialize;
use std::error::Error as StdError;
use thiserror::Error;

pub trait SpecificError: StdError + Serialize + specta::Type {}

impl<E: StdError + Serialize + specta::Type> SpecificError for E {}

#[derive(Error, Debug, Serialize, specta::Type)]
#[serde(tag = "type", content = "error")]
pub enum AliyunRequestCommandError<E: SpecificError> {
    #[error("{}", .0)]
    Specific(#[source] E),

    #[error("{}", .message)]
    RequestFailure {
        message: String,
        #[serde(skip)]
        kind: RequestErrorKind,
        #[serde(skip)]
        source: reqwest::Error,
    },

    #[error("{}", .message)]
    InternalError {
        message: String,
        #[serde(skip)]
        source: Box<dyn StdError>,
    },
}

impl<E: SpecificError> AliyunRequestCommandError<E> {
    pub fn new_specific(error: E) -> Self {
        Self::Specific(error)
    }
}

#[derive(Debug, Error, Serialize)]
#[serde(transparent)]
#[error("unexpected aliyun rejection")]
pub struct UnexpectedAliyunRejectionError(AliyunRejection);

impl<E: SpecificError> From<OperationError> for AliyunRequestCommandError<E> {
    fn from(value: OperationError) -> Self {
        match value {
            // Unhandled aliyun rejection should be converted into InternalError
            OperationError::Rejected(rejection) => {
                let error = UnexpectedAliyunRejectionError(rejection);
                let message = error.to_string();
                Self::InternalError {
                    message,
                    source: Box::new(error),
                }
            }
            OperationError::RequestFailure {
                message,
                kind,
                source,
            } => Self::RequestFailure {
                message,
                kind,
                source,
            },
            OperationError::InternalError { message, source } => {
                Self::InternalError { message, source }
            }
        }
    }
}

#[derive(specta::Type)]
#[allow(dead_code)]
pub struct AliyunRejectionTypeShadow {
    pub code: String,
    pub host_id: String,
    pub message: String,
    pub request_id: String,
    pub recommend: String,
}
