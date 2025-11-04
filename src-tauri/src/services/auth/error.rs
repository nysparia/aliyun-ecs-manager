use alibabacloud::client::error::AliyunRejection;
use serde::Serialize;
use specta::datatype::LiteralType;
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
#[error("the access key credentials provided is not valid: {}", .data.code)]
pub struct AKNotValid {
    r#type: AKNotValidType,
    #[specta(type = AliyunRejectionTypeShadow)]
    pub data: AliyunRejection,
}

impl AKNotValid {
    pub fn new(data: AliyunRejection) -> Self {
        Self {
            r#type: AKNotValidType::default(),
            data,
        }
    }
}

const AK_NOT_VALID_TYPE: &str = "AKNotValid";

#[derive(Debug, Default)]
pub struct AKNotValidType;

impl serde::Serialize for AKNotValidType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(AK_NOT_VALID_TYPE)
    }
}

impl specta::Type for AKNotValidType {
    fn inline(
        _type_map: &mut specta::TypeCollection,
        _generics: specta::Generics,
    ) -> specta::datatype::DataType {
        specta::DataType::Literal(LiteralType::String(AK_NOT_VALID_TYPE.to_owned()))
    }
}

impl From<SaveCredentialError> for AliyunRequestCommandError<AKNotValid> {
    fn from(value: SaveCredentialError) -> Self {
        AliyunRequestCommandError::InternalError {
            message: value.to_string(),
            source: Box::new(value),
        }
    }
}

pub type AKValidationError = AliyunRequestCommandError<AKNotValid>;
pub type AKFulfillError = AliyunRequestCommandError<AKNotValid>;
