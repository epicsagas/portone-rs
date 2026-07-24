use crate::adapter::generated_api::default_api;
use crate::adapter::http_client::HttpClient;
use crate::domain::models;
use crate::port::BillingKeyPort;
use async_trait::async_trait;

#[async_trait]
impl BillingKeyPort for HttpClient {
    async fn confirm_billing_key(
        &self,
        confirm_billing_key_body: models::ConfirmBillingKeyBody,
    ) -> Result<models::ConfirmedBillingKeySummary, crate::error::SdkError> {
        default_api::confirm_billing_key(&self.configuration, confirm_billing_key_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn confirm_billing_key_issue_and_pay(
        &self,
        confirm_billing_key_issue_and_pay_body: models::ConfirmBillingKeyIssueAndPayBody,
    ) -> Result<models::ConfirmedBillingKeyIssueAndPaySummary, crate::error::SdkError> {
        default_api::confirm_billing_key_issue_and_pay(
            &self.configuration,
            confirm_billing_key_issue_and_pay_body,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn delete_billing_key(
        &self,
        billing_key: &str,
        store_id: Option<&str>,
        reason: Option<&str>,
        requester: Option<models::BillingKeyDeleteRequester>,
        skip_webhook: Option<bool>,
    ) -> Result<models::DeleteBillingKeyResponse, crate::error::SdkError> {
        default_api::delete_billing_key(
            &self.configuration,
            billing_key,
            store_id,
            reason,
            requester,
            skip_webhook,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn get_billing_key_info(
        &self,
        billing_key: &str,
        store_id: Option<&str>,
    ) -> Result<models::BillingKeyInfo, crate::error::SdkError> {
        default_api::get_billing_key_info(&self.configuration, billing_key, store_id)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_billing_key_infos(
        &self,
        request_body: Option<models::GetBillingKeyInfosBody>,
    ) -> Result<models::GetBillingKeyInfosResponse, crate::error::SdkError> {
        default_api::get_billing_key_infos(&self.configuration, request_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn issue_billing_key(
        &self,
        issue_billing_key_body: models::IssueBillingKeyBody,
    ) -> Result<models::IssueBillingKeyResponse, crate::error::SdkError> {
        default_api::issue_billing_key(&self.configuration, issue_billing_key_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn pay_with_billing_key(
        &self,
        payment_id: &str,
        billing_key_payment_input: models::BillingKeyPaymentInput,
    ) -> Result<models::PayWithBillingKeyResponse, crate::error::SdkError> {
        default_api::pay_with_billing_key(
            &self.configuration,
            payment_id,
            billing_key_payment_input,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
}
