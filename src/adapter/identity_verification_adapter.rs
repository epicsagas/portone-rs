use crate::adapter::generated_api::default_api;
use crate::adapter::http_client::HttpClient;
use crate::domain::models;
use crate::port::IdentityVerificationPort;
use async_trait::async_trait;

#[async_trait]
impl IdentityVerificationPort for HttpClient {
    async fn confirm_identity_verification(
        &self,
        identity_verification_id: &str,
        confirm_identity_verification_body: models::ConfirmIdentityVerificationBody,
    ) -> Result<models::ConfirmIdentityVerificationResponse, crate::error::SdkError> {
        default_api::confirm_identity_verification(
            &self.configuration,
            identity_verification_id,
            confirm_identity_verification_body,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn get_identity_verification(
        &self,
        identity_verification_id: &str,
        store_id: Option<&str>,
    ) -> Result<models::IdentityVerification, crate::error::SdkError> {
        default_api::get_identity_verification(
            &self.configuration,
            identity_verification_id,
            store_id,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn get_identity_verifications(
        &self,
        request_body: Option<models::GetIdentityVerificationsBody>,
    ) -> Result<models::GetIdentityVerificationsResponse, crate::error::SdkError> {
        default_api::get_identity_verifications(&self.configuration, request_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn resend_identity_verification(
        &self,
        identity_verification_id: &str,
        store_id: Option<&str>,
    ) -> Result<serde_json::Value, crate::error::SdkError> {
        default_api::resend_identity_verification(
            &self.configuration,
            identity_verification_id,
            store_id,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn send_identity_verification(
        &self,
        identity_verification_id: &str,
        send_identity_verification_body: models::SendIdentityVerificationBody,
    ) -> Result<serde_json::Value, crate::error::SdkError> {
        default_api::send_identity_verification(
            &self.configuration,
            identity_verification_id,
            send_identity_verification_body,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
}
