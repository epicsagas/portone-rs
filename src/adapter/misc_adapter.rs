use crate::adapter::generated_api::default_api;
use crate::adapter::http_client::HttpClient;
use crate::domain::models;
use crate::port::MiscPort;
use async_trait::async_trait;

#[async_trait]
impl MiscPort for HttpClient {
    async fn apply_escrow_logistics(
        &self,
        payment_id: &str,
        register_escrow_logistics_body: models::RegisterEscrowLogisticsBody,
    ) -> Result<models::ApplyEscrowLogisticsResponse, crate::error::SdkError> {
        default_api::apply_escrow_logistics(
            &self.configuration,
            payment_id,
            register_escrow_logistics_body,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn close_virtual_account(
        &self,
        payment_id: &str,
        store_id: Option<&str>,
    ) -> Result<models::CloseVirtualAccountResponse, crate::error::SdkError> {
        default_api::close_virtual_account(&self.configuration, payment_id, store_id)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn confirm_escrow(
        &self,
        payment_id: &str,
        confirm_escrow_body: models::ConfirmEscrowBody,
    ) -> Result<models::ConfirmEscrowResponse, crate::error::SdkError> {
        default_api::confirm_escrow(&self.configuration, payment_id, confirm_escrow_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn connect_bulk_partner_counterparty(
        &self,
        connect_bulk_partner_counterparty_body: models::ConnectBulkPartnerCounterpartyBody,
        test: Option<bool>,
    ) -> Result<models::ConnectBulkPartnerCounterpartyResponse, crate::error::SdkError> {
        default_api::connect_bulk_partner_counterparty(
            &self.configuration,
            connect_bulk_partner_counterparty_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn connect_partner_counterparty(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::ConnectPartnerCounterpartyResponse, crate::error::SdkError> {
        default_api::connect_partner_counterparty(&self.configuration, id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn disconnect_bulk_partner_counterparty(
        &self,
        disconnect_bulk_partner_counterparty_body: models::DisconnectBulkPartnerCounterpartyBody,
        test: Option<bool>,
    ) -> Result<models::DisconnectBulkPartnerCounterpartyResponse, crate::error::SdkError> {
        default_api::disconnect_bulk_partner_counterparty(
            &self.configuration,
            disconnect_bulk_partner_counterparty_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn disconnect_partner_counterparty(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::DisconnectPartnerCounterpartyResponse, crate::error::SdkError> {
        default_api::disconnect_partner_counterparty(&self.configuration, id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn evaluate_checkout_profile(
        &self,
        profile_key: &str,
        country: models::Country,
        currency: models::Currency,
        amount: i64,
    ) -> Result<models::EvaluateCheckoutProfileResponse, crate::error::SdkError> {
        default_api::evaluate_checkout_profile(
            &self.configuration,
            profile_key,
            country,
            currency,
            amount,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn get_bank_infos(&self) -> Result<models::GetBankInfosResponse, crate::error::SdkError> {
        default_api::get_bank_infos(&self.configuration)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_cash_receipts(
        &self,
        request_body: Option<models::GetCashReceiptsBody>,
    ) -> Result<models::GetCashReceiptsResponse, crate::error::SdkError> {
        default_api::get_cash_receipts(&self.configuration, request_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_pg_card_promotions(
        &self,
        channel_key: &str,
        amount: i64,
        card_company: Option<models::PgPromotionCardCompany>,
    ) -> Result<models::GetPgCardPromotionsResponse, crate::error::SdkError> {
        default_api::get_pg_card_promotions(&self.configuration, channel_key, amount, card_company)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_promotion(
        &self,
        promotion_id: &str,
    ) -> Result<models::Promotion, crate::error::SdkError> {
        default_api::get_promotion(&self.configuration, promotion_id)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn issue_cash_receipt(
        &self,
        issue_cash_receipt_body: models::IssueCashReceiptBody,
    ) -> Result<models::IssueCashReceiptResponse, crate::error::SdkError> {
        default_api::issue_cash_receipt(&self.configuration, issue_cash_receipt_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn login_via_api_secret(
        &self,
        login_via_api_secret_body: models::LoginViaApiSecretBody,
    ) -> Result<models::LoginViaApiSecretResponse, crate::error::SdkError> {
        default_api::login_via_api_secret(&self.configuration, login_via_api_secret_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn modify_escrow_logistics(
        &self,
        payment_id: &str,
        modify_escrow_logistics_body: models::ModifyEscrowLogisticsBody,
    ) -> Result<models::ModifyEscrowLogisticsResponse, crate::error::SdkError> {
        default_api::modify_escrow_logistics(
            &self.configuration,
            payment_id,
            modify_escrow_logistics_body,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn pay_instantly(
        &self,
        payment_id: &str,
        instant_payment_input: models::InstantPaymentInput,
    ) -> Result<models::PayInstantlyResponse, crate::error::SdkError> {
        default_api::pay_instantly(&self.configuration, payment_id, instant_payment_input)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn refresh_token(
        &self,
        refresh_token_body: models::RefreshTokenBody,
    ) -> Result<models::RefreshTokenResponse, crate::error::SdkError> {
        default_api::refresh_token(&self.configuration, refresh_token_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn register_store_receipt(
        &self,
        payment_id: &str,
        register_store_receipt_body: models::RegisterStoreReceiptBody,
    ) -> Result<models::RegisterStoreReceiptResponse, crate::error::SdkError> {
        default_api::register_store_receipt(
            &self.configuration,
            payment_id,
            register_store_receipt_body,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn reschedule_additional_fee_policy(
        &self,
        id: &str,
        reschedule_platform_additional_fee_policy_body: models::ReschedulePlatformAdditionalFeePolicyBody,
        test: Option<bool>,
    ) -> Result<models::ReschedulePlatformAdditionalFeePolicyResponse, crate::error::SdkError> {
        default_api::reschedule_additional_fee_policy(
            &self.configuration,
            id,
            reschedule_platform_additional_fee_policy_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn reschedule_contract(
        &self,
        id: &str,
        reschedule_platform_contract_body: models::ReschedulePlatformContractBody,
        test: Option<bool>,
    ) -> Result<models::ReschedulePlatformContractResponse, crate::error::SdkError> {
        default_api::reschedule_contract(
            &self.configuration,
            id,
            reschedule_platform_contract_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn reschedule_discount_share_policy(
        &self,
        id: &str,
        reschedule_platform_discount_share_policy_body: models::ReschedulePlatformDiscountSharePolicyBody,
        test: Option<bool>,
    ) -> Result<models::ReschedulePlatformDiscountSharePolicyResponse, crate::error::SdkError> {
        default_api::reschedule_discount_share_policy(
            &self.configuration,
            id,
            reschedule_platform_discount_share_policy_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn reschedule_partner(
        &self,
        id: &str,
        reschedule_platform_partner_body: models::ReschedulePlatformPartnerBody,
        test: Option<bool>,
    ) -> Result<models::ReschedulePlatformPartnerResponse, crate::error::SdkError> {
        default_api::reschedule_partner(
            &self.configuration,
            id,
            reschedule_platform_partner_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn resend_webhook(
        &self,
        payment_id: &str,
        resend_webhook_body: models::ResendWebhookBody,
    ) -> Result<models::ResendWebhookResponse, crate::error::SdkError> {
        default_api::resend_webhook(&self.configuration, payment_id, resend_webhook_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn schedule_additional_fee_policy(
        &self,
        id: &str,
        schedule_platform_additional_fee_policy_body: models::SchedulePlatformAdditionalFeePolicyBody,
        test: Option<bool>,
    ) -> Result<models::SchedulePlatformAdditionalFeePolicyResponse, crate::error::SdkError> {
        default_api::schedule_additional_fee_policy(
            &self.configuration,
            id,
            schedule_platform_additional_fee_policy_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn schedule_contract(
        &self,
        id: &str,
        schedule_platform_contract_body: models::SchedulePlatformContractBody,
        test: Option<bool>,
    ) -> Result<models::SchedulePlatformContractResponse, crate::error::SdkError> {
        default_api::schedule_contract(
            &self.configuration,
            id,
            schedule_platform_contract_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn schedule_discount_share_policy(
        &self,
        id: &str,
        schedule_platform_discount_share_policy_body: models::SchedulePlatformDiscountSharePolicyBody,
        test: Option<bool>,
    ) -> Result<models::SchedulePlatformDiscountSharePolicyResponse, crate::error::SdkError> {
        default_api::schedule_discount_share_policy(
            &self.configuration,
            id,
            schedule_platform_discount_share_policy_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn schedule_partner(
        &self,
        id: &str,
        schedule_platform_partner_body: models::SchedulePlatformPartnerBody,
        test: Option<bool>,
    ) -> Result<models::SchedulePlatformPartnerResponse, crate::error::SdkError> {
        default_api::schedule_partner(
            &self.configuration,
            id,
            schedule_platform_partner_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
}
