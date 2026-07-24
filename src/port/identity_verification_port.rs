use crate::domain::models;
use async_trait::async_trait;

#[async_trait]
pub trait IdentityVerificationPort {
    async fn confirm_identity_verification(
        &self,
        identity_verification_id: &str,
        confirm_identity_verification_body: models::ConfirmIdentityVerificationBody,
    ) -> Result<models::ConfirmIdentityVerificationResponse, crate::error::SdkError>;
    async fn get_identity_verification(
        &self,
        identity_verification_id: &str,
        store_id: Option<&str>,
    ) -> Result<models::IdentityVerification, crate::error::SdkError>;
    async fn get_identity_verifications(
        &self,
        request_body: Option<models::GetIdentityVerificationsBody>,
    ) -> Result<models::GetIdentityVerificationsResponse, crate::error::SdkError>;
    /// ⚠️ 응답 스키마가 미정의(빈 object)되어 raw JSON(`serde_json::Value`)을 반환합니다. PortOne OpenAPI 스펙 한계.
    async fn resend_identity_verification(
        &self,
        identity_verification_id: &str,
        store_id: Option<&str>,
    ) -> Result<serde_json::Value, crate::error::SdkError>;
    /// ⚠️ 응답 스키마가 미정의(빈 object)되어 raw JSON(`serde_json::Value`)을 반환합니다. PortOne OpenAPI 스펙 한계.
    async fn send_identity_verification(
        &self,
        identity_verification_id: &str,
        send_identity_verification_body: models::SendIdentityVerificationBody,
    ) -> Result<serde_json::Value, crate::error::SdkError>;
}
