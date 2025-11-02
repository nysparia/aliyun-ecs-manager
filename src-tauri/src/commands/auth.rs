use serde::Serialize;
use tauri::State;

use crate::services::auth::{
    store::QueryCredentialError as ServiceQueryError, types::AccessKeyCredentials, AccessKeyAuthService,
};

#[derive(thiserror::Error, Debug, Serialize, specta::Type)]
pub enum QueryError {
    #[error("internal query error")]
    Internal(#[from] ServiceQueryError),
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
