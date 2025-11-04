pub mod error;
pub mod store;
pub mod types;

use alibabacloud::client::{
    error::OperationError, sts::caller_identity::CallerIdentityBody, AliyunClient,
};

use crate::services::{
    auth::{
        error::AKNotValid,
        store::{AccessKeyAuthStore, AuthStore, QueryCredentialError},
        types::AccessKeyCredentials,
    },
    error::AliyunRequestCommandError,
};

pub struct AccessKeyAuthService {
    auth_store: Box<dyn AuthStore<AccessKeyCredentials> + Send + Sync>,
}

impl AccessKeyAuthService {
    pub fn new<R: tauri::Runtime + 'static, S: Into<AccessKeyAuthStore<R>>>(store: S) -> Self {
        Self {
            auth_store: Box::new(store.into()),
        }
    }

    pub fn current_access_key_credentials(
        &self,
    ) -> Result<AccessKeyCredentials, QueryCredentialError> {
        self.auth_store.query()
    }

    pub async fn get_credentials_status(&self) {}

    pub async fn validate_access_key_credentials(
        credentials: AccessKeyCredentials,
    ) -> Result<CallerIdentityBody, AliyunRequestCommandError<AKNotValid>> {
        use OperationError::*;

        let client = AliyunClient::new(credentials.access_key_id, credentials.access_key_secret);
        client
            .sts()
            .get_caller_identity()
            .await
            .map_err(|err| match err {
                // todo: use match if to simplify branches
                Rejected(aliyun_rejection) => {
                    let code = &aliyun_rejection.code;
                    let main_code = code.split_once(".").unwrap_or((&code, "")).0;
                    log::debug!(
                        "Received aliyun error code: {:#?}, read: {:#?}",
                        code,
                        main_code
                    );

                    if main_code == "InvalidAccessKeyId"
                        || main_code == "SignatureDoesNotMatch"
                        || main_code == "MissingAccessKeyId"
                    {
                        AliyunRequestCommandError::<AKNotValid>::new_specific(AKNotValid(
                            aliyun_rejection,
                        ))
                    } else {
                        OperationError::Rejected(aliyun_rejection).into()
                    }
                }
                other => other.into(),
            })
    }

    pub async fn fulfill_access_key_credentials(
        &self,
        credentials: AccessKeyCredentials,
    ) -> Result<CallerIdentityBody, AliyunRequestCommandError<AKNotValid>> {
        let caller_identity = Self::validate_access_key_credentials(credentials.clone()).await?;
        self.auth_store.save(credentials)?;
        Ok(caller_identity)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use claims::assert_matches;
    use once_cell::sync::Lazy;
    use pretty_assertions::assert_eq;

    use crate::services::auth::store::store_test_utils;

    use super::*;

    #[derive(Debug, thiserror::Error)]
    #[error("Failed to load test credentials from environment variables")]
    pub struct TestSecretsError(#[from] env::VarError);

    impl AccessKeyCredentials {
        pub fn from_env() -> Result<Self, TestSecretsError> {
            // It is recommended to store test credentials in a `.env` file at the project root
            // for local development. Environment variables set in `.cargo/config.toml`
            // will override corresponding values from the `.env` file.
            //
            // Note: some IDEs and their debuggers may not load environment variables from
            // `.cargo/config.toml`, which can lead to different behavior when running or
            // debugging tests inside the IDE.
            let _ = dotenv::dotenv();

            let access_key_id = env::var("TEST_ACCESS_KEY_ID")?;
            let access_key_secret = env::var("TEST_ACCESS_KEY_SECRET")?;

            Ok(Self {
                access_key_id,
                access_key_secret,
            })
        }
    }

    static RIGHT_ACCESS_KEY_CREDENTIALS: Lazy<AccessKeyCredentials> = Lazy::new(|| {
        AccessKeyCredentials::from_env().expect("Failed to load test secrets from environment")
    });

    #[tokio::test]
    async fn test_validate_access_key_credential() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();

        // Invalid credentials
        for (creds, expected_code) in [
            (AccessKeyCredentials::new("", ""), "MissingAccessKeyId"),
            (
                AccessKeyCredentials::new("invalid", "invalid"),
                "InvalidAccessKeyId.NotFound",
            ),
            (
                AccessKeyCredentials::new(
                    "invalid",
                    &RIGHT_ACCESS_KEY_CREDENTIALS.access_key_secret,
                ),
                "InvalidAccessKeyId.NotFound",
            ),
            (
                AccessKeyCredentials::new(&RIGHT_ACCESS_KEY_CREDENTIALS.access_key_id, "invalid"),
                "SignatureDoesNotMatch",
            ),
            (
                AccessKeyCredentials::new(
                    &RIGHT_ACCESS_KEY_CREDENTIALS.access_key_secret,
                    &RIGHT_ACCESS_KEY_CREDENTIALS.access_key_id,
                ),
                "InvalidAccessKeyId.NotFound",
            ),
        ] {
            log::info!(
                "Testing invalid credentials: {:?}, expect: {:?}",
                creds,
                expected_code
            );
            let result = AccessKeyAuthService::validate_access_key_credentials(creds).await;

            let Err(AliyunRequestCommandError::Specific(err)) = result else {
                unreachable!()
            };

            assert_eq!(err.0.code, expected_code);
        }

        // Valid credentials
        let result = AccessKeyAuthService::validate_access_key_credentials(
            RIGHT_ACCESS_KEY_CREDENTIALS.clone(),
        )
        .await;

        let Ok(body) = result else { unreachable!() };

        println!("{:?}", body);
    }

    #[tokio::test]
    async fn test_fulfill_access_key_credentials() {
        let auth_store = store_test_utils::init_auth_store();
        let auth_service = AccessKeyAuthService::new(auth_store);

        let current_credentials = auth_service.current_access_key_credentials().unwrap_err();
        assert_matches!(current_credentials, QueryCredentialError::NotExist);

        let caller_identity = auth_service
            .fulfill_access_key_credentials(RIGHT_ACCESS_KEY_CREDENTIALS.clone())
            .await
            .unwrap();

        println!("{:?}", caller_identity);

        let current_credentials = auth_service.current_access_key_credentials().unwrap();
        assert_eq!(current_credentials, RIGHT_ACCESS_KEY_CREDENTIALS.clone());
    }
}
