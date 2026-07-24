use crate::adapter::generated_api::default_api;
use crate::adapter::http_client::HttpClient;
use crate::domain::models;
use crate::port::PlatformPort;
use async_trait::async_trait;

#[async_trait]
impl PlatformPort for HttpClient {
    async fn archive_platform_additional_fee_policy(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::ArchivePlatformAdditionalFeePolicyResponse, crate::error::SdkError> {
        default_api::archive_platform_additional_fee_policy(&self.configuration, id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn archive_platform_contract(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::ArchivePlatformContractResponse, crate::error::SdkError> {
        default_api::archive_platform_contract(&self.configuration, id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn archive_platform_discount_share_policy(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::ArchivePlatformDiscountSharePolicyResponse, crate::error::SdkError> {
        default_api::archive_platform_discount_share_policy(&self.configuration, id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn archive_platform_partner(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::ArchivePlatformPartnerResponse, crate::error::SdkError> {
        default_api::archive_platform_partner(&self.configuration, id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn cancel_platform_additional_fee_policy_schedule(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<serde_json::Value, crate::error::SdkError> {
        default_api::cancel_platform_additional_fee_policy_schedule(&self.configuration, id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn cancel_platform_contract_schedule(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<serde_json::Value, crate::error::SdkError> {
        default_api::cancel_platform_contract_schedule(&self.configuration, id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn cancel_platform_discount_share_policy_schedule(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<serde_json::Value, crate::error::SdkError> {
        default_api::cancel_platform_discount_share_policy_schedule(&self.configuration, id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn cancel_platform_partner_schedule(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<serde_json::Value, crate::error::SdkError> {
        default_api::cancel_platform_partner_schedule(&self.configuration, id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn complete_platform_payout_by_partner_settlement_ids(
        &self,
        complete_platform_payout_by_partner_settlement_ids_body: models::CompletePlatformPayoutByPartnerSettlementIdsBody,
        test: Option<bool>,
    ) -> Result<models::CompletePlatformPayoutByPartnerSettlementIdsResponse, crate::error::SdkError>
    {
        default_api::complete_platform_payout_by_partner_settlement_ids(
            &self.configuration,
            complete_platform_payout_by_partner_settlement_ids_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn create_platform_additional_fee_policy(
        &self,
        create_platform_additional_fee_policy_body: models::CreatePlatformAdditionalFeePolicyBody,
        test: Option<bool>,
    ) -> Result<models::CreatePlatformAdditionalFeePolicyResponse, crate::error::SdkError> {
        default_api::create_platform_additional_fee_policy(
            &self.configuration,
            create_platform_additional_fee_policy_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn create_platform_contract(
        &self,
        create_platform_contract_body: models::CreatePlatformContractBody,
        test: Option<bool>,
    ) -> Result<models::CreatePlatformContractResponse, crate::error::SdkError> {
        default_api::create_platform_contract(
            &self.configuration,
            create_platform_contract_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn create_platform_discount_share_policy(
        &self,
        create_platform_discount_share_policy_body: models::CreatePlatformDiscountSharePolicyBody,
        test: Option<bool>,
    ) -> Result<models::CreatePlatformDiscountSharePolicyResponse, crate::error::SdkError> {
        default_api::create_platform_discount_share_policy(
            &self.configuration,
            create_platform_discount_share_policy_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn create_platform_manual_transfer(
        &self,
        create_platform_manual_transfer_body: models::CreatePlatformManualTransferBody,
        test: Option<bool>,
    ) -> Result<models::CreateManualTransferResponse, crate::error::SdkError> {
        default_api::create_platform_manual_transfer(
            &self.configuration,
            create_platform_manual_transfer_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn create_platform_order_cancel_transfer(
        &self,
        create_platform_order_cancel_transfer_body: models::CreatePlatformOrderCancelTransferBody,
        test: Option<bool>,
    ) -> Result<models::CreateOrderCancelTransferResponse, crate::error::SdkError> {
        default_api::create_platform_order_cancel_transfer(
            &self.configuration,
            create_platform_order_cancel_transfer_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn create_platform_order_transfer(
        &self,
        create_platform_order_transfer_body: models::CreatePlatformOrderTransferBody,
        test: Option<bool>,
    ) -> Result<models::CreateOrderTransferResponse, crate::error::SdkError> {
        default_api::create_platform_order_transfer(
            &self.configuration,
            create_platform_order_transfer_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn create_platform_partner(
        &self,
        create_platform_partner_body: models::CreatePlatformPartnerBody,
        test: Option<bool>,
    ) -> Result<models::CreatePlatformPartnerResponse, crate::error::SdkError> {
        default_api::create_platform_partner(
            &self.configuration,
            create_platform_partner_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn create_platform_partners(
        &self,
        create_platform_partners_body: models::CreatePlatformPartnersBody,
        test: Option<bool>,
    ) -> Result<models::CreatePlatformPartnersResponse, crate::error::SdkError> {
        default_api::create_platform_partners(
            &self.configuration,
            create_platform_partners_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn delete_platform_partner_settlements(
        &self,
        delete_platform_partner_settlements_body: models::DeletePlatformPartnerSettlementsBody,
        test: Option<bool>,
    ) -> Result<serde_json::Value, crate::error::SdkError> {
        default_api::delete_platform_partner_settlements(
            &self.configuration,
            delete_platform_partner_settlements_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn delete_platform_transfer(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<serde_json::Value, crate::error::SdkError> {
        default_api::delete_platform_transfer(&self.configuration, id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn download_platform_transfer_sheet(
        &self,
        request_body: models::DownloadPlatformTransferSheetBody,
        test: Option<bool>,
    ) -> Result<reqwest::Response, crate::error::SdkError> {
        default_api::download_platform_transfer_sheet(&self.configuration, request_body, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_platform_account_holder(
        &self,
        bank: models::Bank,
        account_number: &str,
        test: Option<bool>,
        birthdate: Option<&str>,
        business_registration_number: Option<&str>,
    ) -> Result<models::PlatformAccountHolder, crate::error::SdkError> {
        default_api::get_platform_account_holder(
            &self.configuration,
            bank,
            account_number,
            test,
            birthdate,
            business_registration_number,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn get_platform_account_transfers(
        &self,
        test: Option<bool>,
        request_body: Option<models::GetAccountTransfersBody1>,
    ) -> Result<models::GetPlatformAccountTransfersResponse, crate::error::SdkError> {
        default_api::get_platform_account_transfers(&self.configuration, test, request_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_platform_additional_fee_policies(
        &self,
        test: Option<bool>,
        request_body: Option<models::GetPlatformAdditionalFeePoliciesBody>,
    ) -> Result<models::GetPlatformAdditionalFeePoliciesResponse, crate::error::SdkError> {
        default_api::get_platform_additional_fee_policies(&self.configuration, test, request_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_platform_additional_fee_policy(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::PlatformAdditionalFeePolicy, crate::error::SdkError> {
        default_api::get_platform_additional_fee_policy(&self.configuration, id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_platform_additional_fee_policy_schedule(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::PlatformAdditionalFeePolicy, crate::error::SdkError> {
        default_api::get_platform_additional_fee_policy_schedule(&self.configuration, id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_platform_bulk_account_transfers(
        &self,
        test: Option<bool>,
        request_body: Option<models::GetPlatformBulkAccountTransfersBody>,
    ) -> Result<models::GetPlatformBulkAccountTransfersResponse, crate::error::SdkError> {
        default_api::get_platform_bulk_account_transfers(&self.configuration, test, request_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_platform_bulk_payouts(
        &self,
        test: Option<bool>,
        request_body: Option<models::GetPlatformBulkPayoutsBody>,
    ) -> Result<models::GetPlatformBulkPayoutsResponse, crate::error::SdkError> {
        default_api::get_platform_bulk_payouts(&self.configuration, test, request_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_platform_company_state(
        &self,
        business_registration_number: &str,
        test: Option<bool>,
    ) -> Result<models::GetPlatformCompanyStatePayload, crate::error::SdkError> {
        default_api::get_platform_company_state(
            &self.configuration,
            business_registration_number,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn get_platform_contract(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::PlatformContract, crate::error::SdkError> {
        default_api::get_platform_contract(&self.configuration, id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_platform_contract_schedule(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::PlatformContract, crate::error::SdkError> {
        default_api::get_platform_contract_schedule(&self.configuration, id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_platform_contracts(
        &self,
        test: Option<bool>,
        request_body: Option<models::GetPlatformContractsBody>,
    ) -> Result<models::GetPlatformContractsResponse, crate::error::SdkError> {
        default_api::get_platform_contracts(&self.configuration, test, request_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_platform_discount_share_policies(
        &self,
        test: Option<bool>,
        request_body: Option<models::GetPlatformDiscountSharePoliciesBody>,
    ) -> Result<models::GetPlatformDiscountSharePoliciesResponse, crate::error::SdkError> {
        default_api::get_platform_discount_share_policies(&self.configuration, test, request_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_platform_discount_share_policy(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::PlatformDiscountSharePolicy, crate::error::SdkError> {
        default_api::get_platform_discount_share_policy(&self.configuration, id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_platform_discount_share_policy_filter_options(
        &self,
        test: Option<bool>,
        is_archived: Option<bool>,
    ) -> Result<models::PlatformDiscountSharePolicyFilterOptions, crate::error::SdkError> {
        default_api::get_platform_discount_share_policy_filter_options(
            &self.configuration,
            test,
            is_archived,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn get_platform_discount_share_policy_schedule(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::PlatformDiscountSharePolicy, crate::error::SdkError> {
        default_api::get_platform_discount_share_policy_schedule(&self.configuration, id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_platform_partner(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::PlatformPartner, crate::error::SdkError> {
        default_api::get_platform_partner(&self.configuration, id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_platform_partner_filter_options(
        &self,
        test: Option<bool>,
        is_archived: Option<bool>,
    ) -> Result<models::PlatformPartnerFilterOptions, crate::error::SdkError> {
        default_api::get_platform_partner_filter_options(&self.configuration, test, is_archived)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_platform_partner_schedule(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::PlatformPartner, crate::error::SdkError> {
        default_api::get_platform_partner_schedule(&self.configuration, id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_platform_partner_settlements(
        &self,
        test: Option<bool>,
        request_body: Option<models::GetPlatformPartnerSettlementsBody>,
    ) -> Result<models::GetPlatformPartnerSettlementsResponse, crate::error::SdkError> {
        default_api::get_platform_partner_settlements(&self.configuration, test, request_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_platform_partners(
        &self,
        test: Option<bool>,
        request_body: Option<models::GetPlatformPartnersBody>,
    ) -> Result<models::GetPlatformPartnersResponse, crate::error::SdkError> {
        default_api::get_platform_partners(&self.configuration, test, request_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_platform_payouts(
        &self,
        test: Option<bool>,
        request_body: Option<models::GetPlatformPayoutsBody>,
    ) -> Result<models::GetPlatformPayoutsResponse, crate::error::SdkError> {
        default_api::get_platform_payouts(&self.configuration, test, request_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_platform_setting(
        &self,
        test: Option<bool>,
    ) -> Result<models::PlatformSetting, crate::error::SdkError> {
        default_api::get_platform_setting(&self.configuration, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_platform_transfer(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::PlatformTransfer, crate::error::SdkError> {
        default_api::get_platform_transfer(&self.configuration, id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_platform_transfer_summaries(
        &self,
        test: Option<bool>,
        request_body: Option<models::GetPlatformTransferSummariesBody>,
    ) -> Result<models::GetPlatformTransferSummariesResponse, crate::error::SdkError> {
        default_api::get_platform_transfer_summaries(&self.configuration, test, request_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn recover_platform_additional_fee_policy(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::RecoverPlatformAdditionalFeePolicyResponse, crate::error::SdkError> {
        default_api::recover_platform_additional_fee_policy(&self.configuration, id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn recover_platform_contract(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::RecoverPlatformContractResponse, crate::error::SdkError> {
        default_api::recover_platform_contract(&self.configuration, id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn recover_platform_discount_share_policy(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::RecoverPlatformDiscountSharePolicyResponse, crate::error::SdkError> {
        default_api::recover_platform_discount_share_policy(&self.configuration, id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn recover_platform_partner(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::RecoverPlatformPartnerResponse, crate::error::SdkError> {
        default_api::recover_platform_partner(&self.configuration, id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn schedule_platform_partners(
        &self,
        schedule_platform_partners_body: models::SchedulePlatformPartnersBody,
        test: Option<bool>,
    ) -> Result<serde_json::Value, crate::error::SdkError> {
        default_api::schedule_platform_partners(
            &self.configuration,
            schedule_platform_partners_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn update_platform_additional_fee_policy(
        &self,
        id: &str,
        update_platform_additional_fee_policy_body: models::UpdatePlatformAdditionalFeePolicyBody,
        test: Option<bool>,
    ) -> Result<models::UpdatePlatformAdditionalFeePolicyResponse, crate::error::SdkError> {
        default_api::update_platform_additional_fee_policy(
            &self.configuration,
            id,
            update_platform_additional_fee_policy_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn update_platform_contract(
        &self,
        id: &str,
        update_platform_contract_body: models::UpdatePlatformContractBody,
        test: Option<bool>,
    ) -> Result<models::UpdatePlatformContractResponse, crate::error::SdkError> {
        default_api::update_platform_contract(
            &self.configuration,
            id,
            update_platform_contract_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn update_platform_discount_share_policy(
        &self,
        id: &str,
        update_platform_discount_share_policy_body: models::UpdatePlatformDiscountSharePolicyBody,
        test: Option<bool>,
    ) -> Result<models::UpdatePlatformDiscountSharePolicyResponse, crate::error::SdkError> {
        default_api::update_platform_discount_share_policy(
            &self.configuration,
            id,
            update_platform_discount_share_policy_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn update_platform_partner(
        &self,
        id: &str,
        update_platform_partner_body: models::UpdatePlatformPartnerBody,
        test: Option<bool>,
    ) -> Result<models::UpdatePlatformPartnerResponse, crate::error::SdkError> {
        default_api::update_platform_partner(
            &self.configuration,
            id,
            update_platform_partner_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn update_platform_setting(
        &self,
        update_platform_setting_body: models::UpdatePlatformSettingBody,
        test: Option<bool>,
    ) -> Result<models::UpdatePlatformSettingResponse, crate::error::SdkError> {
        default_api::update_platform_setting(
            &self.configuration,
            update_platform_setting_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
}
