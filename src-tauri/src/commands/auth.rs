use alibabacloud::client::sts::caller_identity::{CallerIdentityBody, IdentityType};
use serde::Serialize;
use tauri::State;

use crate::services::{
    auth::{
        error::{AKFulfillError, AKValidationError},
        store::QueryCredentialError as ServiceQueryError,
        types::AccessKeyCredentials,
        AccessKeyAuthService,
    },
    client::{AliyunClientService, ClientValidationError},
    error::NoSource,
};

#[derive(thiserror::Error, Debug, Serialize, specta::Type)]
#[serde(tag = "type", content = "error")]
pub enum QueryError {
    #[error("internal query error")]
    UnderlyingError(#[from] ServiceQueryError),
}

#[tauri::command]
#[specta::specta]
/// Retrieve the currently stored access key credential, if any.
///
/// This command queries the authentication service for the current
/// `AccessKeyCredential`. It returns `Ok(Some(credential))` when a
/// credential is available, `Ok(None)` when no credential is stored,
/// and `Err(QueryError::Internal(_))` for all other service-level
/// failures. The documentation intentionally omits details about the
/// service's internal error variants.
///
/// # Errors
///
/// Returns `Err(QueryError::Internal(_))` for unexpected or internal
/// failures from the authentication service.
///
/// # Examples
///
/// ```rust,ignore
/// // from a Tauri frontend
/// let cred = invoke("current_access_key_credential");
/// ```
pub fn current_access_key_credential(
    auth_service: State<AccessKeyAuthService>,
) -> Result<Option<AccessKeyCredentials>, QueryError> {
    auth_service
        .current_access_key_credentials()
        .map(Some)
        .or_else(|err| match err {
            ServiceQueryError::NotExist => Ok(None),
            other => Err(other.into()),
        })
}

#[derive(specta::Type)]
#[allow(dead_code)]
enum IdentityTypeShadow {
    Account,
    RAMUser,
    AssumedRoleUser,
}

#[derive(specta::Type)]
#[allow(dead_code)]
struct CallerIdentityBodyTypeShadow {
    #[specta(type = IdentityTypeShadow)]
    pub identity_type: IdentityType,
    pub request_id: String,
    pub account_id: String,
    pub principal_id: String,
    pub user_id: String,
    pub arn: String,
    pub role_id: Option<String>,
}

#[derive(Debug, Serialize, specta::Type)]
#[serde(transparent)]
pub struct CallerIdentity(#[specta(type = CallerIdentityBodyTypeShadow)] CallerIdentityBody);

impl From<CallerIdentityBody> for CallerIdentity {
    fn from(value: CallerIdentityBody) -> Self {
        CallerIdentity(value)
    }
}

#[tauri::command]
#[specta::specta]
/// Validate the provided access key credentials.
///
/// This command validates the given `AccessKeyCredentials` by making
/// a request to Aliyun's STS service to retrieve the caller identity.
/// It returns `Ok(CallerIdentityBody)` when the credentials are valid,
/// and `Err(AKValidationError)` when validation fails due to invalid
/// credentials or service errors.
///
/// # Errors
///
/// Returns `Err(AKValidationError)` when the credentials are invalid
/// or when there are failures communicating with the Aliyun service.
///
/// # Examples
///
/// ```rust,ignore
/// // from a Tauri frontend
/// let identity = invoke("validate_access_key_credentials", { credentials });
/// ```
pub async fn validate_access_key_credentials(
    credentials: AccessKeyCredentials,
) -> Result<CallerIdentity, AKValidationError> {
    AccessKeyAuthService::validate_access_key_credentials(credentials)
        .await
        .map(|r| r.into())
}

#[tauri::command]
#[specta::specta]
/// Validate and store the provided access key credentials.
///
/// This command validates the given `AccessKeyCredentials` and, if valid,
/// stores them in the authentication service for future use. It returns
/// `Ok(CallerIdentityBody)` when the credentials are successfully validated
/// and stored, and `Err(AKFulfillError)` when validation or storage fails.
///
/// # Errors
///
/// Returns `Err(AKFulfillError)` when the credentials are invalid,
/// when there are failures communicating with the Aliyun service,
/// or when storing the credentials fails.
///
/// # Examples
///
/// ```rust,ignore
/// // from a Tauri frontend
/// let identity = invoke("fulfill_access_key_credentials", { credentials });
/// ```
pub async fn fulfill_access_key_credentials(
    credentials: AccessKeyCredentials,
    auth_service: State<'_, AccessKeyAuthService>,
    client_service: State<'_, AliyunClientService>,
) -> Result<CallerIdentity, AKFulfillError> {
    let result = auth_service
        .fulfill_access_key_credentials(credentials)
        .await
        .map(|r| r.into())?;

    let Some(client) = auth_service.new_client() else {
        return Err(AKFulfillError::InternalError {
            message: "can't unwrap valid aliyun client when using auth_service.new_client"
                .to_owned(),
            source: NoSource::new_boxed(),
        });
    };
    client_service.initialize(client);

    Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn has_aliyun_client(client_service: State<'_, AliyunClientService>) -> Result<bool, ()> {
    Ok(client_service.is_initialized())
}

#[tauri::command]
#[specta::specta]
/// Check if there is a valid Aliyun client available.
///
/// This command queries the client service to determine whether a valid
/// Aliyun client instance exists and is properly configured. It returns
/// `Ok(true)` when a valid client is available, `Ok(false)` when no client
/// is configured or the client is invalid, and `Err(ClientValidationError)`
/// for validation failures or service errors.
///
/// # Errors
///
/// Returns `Err(ClientValidationError)` when there are failures during
/// the validation process or when communicating with the client service.
///
/// # Examples
///
/// ```rust,ignore
/// // from a Tauri frontend
/// let is_valid = invoke("has_valid_aliyun_client");
/// ```
pub async fn has_valid_aliyun_client(
    client_service: State<'_, AliyunClientService>,
) -> Result<bool, ClientValidationError> {
    client_service.is_valid().await
}
