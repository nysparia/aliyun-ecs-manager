pub mod store;
pub mod types;

use alibabacloud::client::{
    error::AdvancedClientError, sts::caller_identity::CallerIdentityBody, AliyunClient,
};

use crate::{
    services::auth::{
        store::{AccessKeyAuthStore, AuthStore, QueryCredentialError},
        types::AccessKeyCredentials,
    },
    types::Store,
};

pub struct AccessKeyAuthService {
    auth_store: Box<dyn AuthStore<AccessKeyCredentials> + Send + Sync>,
}

impl AccessKeyAuthService {
    pub fn new<R: tauri::Runtime + 'static>(store: Store<R>) -> Self {
        Self {
            auth_store: Box::new(AccessKeyAuthStore::<R>::new(store)),
        }
    }

    pub fn current_access_key_credential(
        &self,
    ) -> Result<AccessKeyCredentials, QueryCredentialError> {
        self.auth_store.query()
    }

    pub async fn validate_access_key_credential(
        credential: AccessKeyCredentials,
    ) -> Result<CallerIdentityBody, AdvancedClientError> {
        let client = AliyunClient::new(credential.access_key_id, credential.access_key_secret);
        let result = client.sts().get_caller_identity().await;
        // println!("{:?}", result);
        result
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use once_cell::sync::Lazy;

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
        // Invalid credentials
        let result =
            AccessKeyAuthService::validate_access_key_credential(AccessKeyCredentials::new("", ""))
                .await;

        let Err(AdvancedClientError::AliyunRejectError(err)) = result else {
            unreachable!()
        };

        assert_eq!(err.code, "MissingAccessKeyId");

        // Valid credentials
        let result = AccessKeyAuthService::validate_access_key_credential(
            RIGHT_ACCESS_KEY_CREDENTIALS.clone(),
        )
        .await;

        let Ok(body) = result else { unreachable!() };

        println!("{:?}", body);
    }
}
