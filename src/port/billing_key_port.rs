use crate::domain::models;
use async_trait::async_trait;

#[async_trait]
pub trait BillingKeyPort {
    async fn confirm_billing_key(
        &self,
        confirm_billing_key_body: models::ConfirmBillingKeyBody,
    ) -> Result<models::ConfirmedBillingKeySummary, crate::error::SdkError>;
    async fn confirm_billing_key_issue_and_pay(
        &self,
        confirm_billing_key_issue_and_pay_body: models::ConfirmBillingKeyIssueAndPayBody,
    ) -> Result<models::ConfirmedBillingKeyIssueAndPaySummary, crate::error::SdkError>;
    async fn delete_billing_key(
        &self,
        billing_key: &str,
        store_id: Option<&str>,
        reason: Option<&str>,
        requester: Option<models::BillingKeyDeleteRequester>,
        skip_webhook: Option<bool>,
    ) -> Result<models::DeleteBillingKeyResponse, crate::error::SdkError>;
    async fn get_billing_key_info(
        &self,
        billing_key: &str,
        store_id: Option<&str>,
    ) -> Result<models::BillingKeyInfo, crate::error::SdkError>;
    async fn get_billing_key_infos(
        &self,
        request_body: Option<models::GetBillingKeyInfosBody>,
    ) -> Result<models::GetBillingKeyInfosResponse, crate::error::SdkError>;
    async fn issue_billing_key(
        &self,
        issue_billing_key_body: models::IssueBillingKeyBody,
    ) -> Result<models::IssueBillingKeyResponse, crate::error::SdkError>;
    async fn pay_with_billing_key(
        &self,
        payment_id: &str,
        billing_key_payment_input: models::BillingKeyPaymentInput,
    ) -> Result<models::PayWithBillingKeyResponse, crate::error::SdkError>;
}
