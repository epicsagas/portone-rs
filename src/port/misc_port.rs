use crate::domain::models;
use async_trait::async_trait;

#[async_trait]
pub trait MiscPort {
    async fn apply_escrow_logistics(
        &self,
        payment_id: &str,
        register_escrow_logistics_body: models::RegisterEscrowLogisticsBody,
    ) -> Result<models::ApplyEscrowLogisticsResponse, crate::error::SdkError>;
    async fn close_virtual_account(
        &self,
        payment_id: &str,
        store_id: Option<&str>,
    ) -> Result<models::CloseVirtualAccountResponse, crate::error::SdkError>;
    async fn confirm_escrow(
        &self,
        payment_id: &str,
        confirm_escrow_body: models::ConfirmEscrowBody,
    ) -> Result<models::ConfirmEscrowResponse, crate::error::SdkError>;
    async fn connect_bulk_partner_counterparty(
        &self,
        connect_bulk_partner_counterparty_body: models::ConnectBulkPartnerCounterpartyBody,
        test: Option<bool>,
    ) -> Result<models::ConnectBulkPartnerCounterpartyResponse, crate::error::SdkError>;
    async fn connect_partner_counterparty(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::ConnectPartnerCounterpartyResponse, crate::error::SdkError>;
    async fn disconnect_bulk_partner_counterparty(
        &self,
        disconnect_bulk_partner_counterparty_body: models::DisconnectBulkPartnerCounterpartyBody,
        test: Option<bool>,
    ) -> Result<models::DisconnectBulkPartnerCounterpartyResponse, crate::error::SdkError>;
    async fn disconnect_partner_counterparty(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::DisconnectPartnerCounterpartyResponse, crate::error::SdkError>;
    async fn evaluate_checkout_profile(
        &self,
        profile_key: &str,
        country: models::Country,
        currency: models::Currency,
        amount: i64,
    ) -> Result<models::EvaluateCheckoutProfileResponse, crate::error::SdkError>;
    async fn get_bank_infos(&self) -> Result<models::GetBankInfosResponse, crate::error::SdkError>;
    async fn get_cash_receipts(
        &self,
        request_body: Option<models::GetCashReceiptsBody>,
    ) -> Result<models::GetCashReceiptsResponse, crate::error::SdkError>;
    async fn get_pg_card_promotions(
        &self,
        channel_key: &str,
        amount: i64,
        card_company: Option<models::PgPromotionCardCompany>,
    ) -> Result<models::GetPgCardPromotionsResponse, crate::error::SdkError>;
    async fn get_promotion(
        &self,
        promotion_id: &str,
    ) -> Result<models::Promotion, crate::error::SdkError>;
    async fn issue_cash_receipt(
        &self,
        issue_cash_receipt_body: models::IssueCashReceiptBody,
    ) -> Result<models::IssueCashReceiptResponse, crate::error::SdkError>;
    async fn login_via_api_secret(
        &self,
        login_via_api_secret_body: models::LoginViaApiSecretBody,
    ) -> Result<models::LoginViaApiSecretResponse, crate::error::SdkError>;
    async fn modify_escrow_logistics(
        &self,
        payment_id: &str,
        modify_escrow_logistics_body: models::ModifyEscrowLogisticsBody,
    ) -> Result<models::ModifyEscrowLogisticsResponse, crate::error::SdkError>;
    async fn pay_instantly(
        &self,
        payment_id: &str,
        instant_payment_input: models::InstantPaymentInput,
    ) -> Result<models::PayInstantlyResponse, crate::error::SdkError>;
    async fn refresh_token(
        &self,
        refresh_token_body: models::RefreshTokenBody,
    ) -> Result<models::RefreshTokenResponse, crate::error::SdkError>;
    async fn register_store_receipt(
        &self,
        payment_id: &str,
        register_store_receipt_body: models::RegisterStoreReceiptBody,
    ) -> Result<models::RegisterStoreReceiptResponse, crate::error::SdkError>;
    async fn reschedule_additional_fee_policy(
        &self,
        id: &str,
        reschedule_platform_additional_fee_policy_body: models::ReschedulePlatformAdditionalFeePolicyBody,
        test: Option<bool>,
    ) -> Result<models::ReschedulePlatformAdditionalFeePolicyResponse, crate::error::SdkError>;
    async fn reschedule_contract(
        &self,
        id: &str,
        reschedule_platform_contract_body: models::ReschedulePlatformContractBody,
        test: Option<bool>,
    ) -> Result<models::ReschedulePlatformContractResponse, crate::error::SdkError>;
    async fn reschedule_discount_share_policy(
        &self,
        id: &str,
        reschedule_platform_discount_share_policy_body: models::ReschedulePlatformDiscountSharePolicyBody,
        test: Option<bool>,
    ) -> Result<models::ReschedulePlatformDiscountSharePolicyResponse, crate::error::SdkError>;
    async fn reschedule_partner(
        &self,
        id: &str,
        reschedule_platform_partner_body: models::ReschedulePlatformPartnerBody,
        test: Option<bool>,
    ) -> Result<models::ReschedulePlatformPartnerResponse, crate::error::SdkError>;
    async fn resend_webhook(
        &self,
        payment_id: &str,
        resend_webhook_body: models::ResendWebhookBody,
    ) -> Result<models::ResendWebhookResponse, crate::error::SdkError>;
    async fn schedule_additional_fee_policy(
        &self,
        id: &str,
        schedule_platform_additional_fee_policy_body: models::SchedulePlatformAdditionalFeePolicyBody,
        test: Option<bool>,
    ) -> Result<models::SchedulePlatformAdditionalFeePolicyResponse, crate::error::SdkError>;
    async fn schedule_contract(
        &self,
        id: &str,
        schedule_platform_contract_body: models::SchedulePlatformContractBody,
        test: Option<bool>,
    ) -> Result<models::SchedulePlatformContractResponse, crate::error::SdkError>;
    async fn schedule_discount_share_policy(
        &self,
        id: &str,
        schedule_platform_discount_share_policy_body: models::SchedulePlatformDiscountSharePolicyBody,
        test: Option<bool>,
    ) -> Result<models::SchedulePlatformDiscountSharePolicyResponse, crate::error::SdkError>;
    async fn schedule_partner(
        &self,
        id: &str,
        schedule_platform_partner_body: models::SchedulePlatformPartnerBody,
        test: Option<bool>,
    ) -> Result<models::SchedulePlatformPartnerResponse, crate::error::SdkError>;
}
