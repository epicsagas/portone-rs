use crate::domain::models;
use async_trait::async_trait;

#[async_trait]
pub trait PlatformPort {
    async fn archive_platform_additional_fee_policy(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::ArchivePlatformAdditionalFeePolicyResponse, crate::error::SdkError>;
    async fn archive_platform_contract(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::ArchivePlatformContractResponse, crate::error::SdkError>;
    async fn archive_platform_discount_share_policy(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::ArchivePlatformDiscountSharePolicyResponse, crate::error::SdkError>;
    async fn archive_platform_partner(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::ArchivePlatformPartnerResponse, crate::error::SdkError>;
    /// ⚠️ 응답 스키마가 미정의(빈 object)되어 raw JSON(`serde_json::Value`)을 반환합니다. PortOne OpenAPI 스펙 한계.
    async fn cancel_platform_additional_fee_policy_schedule(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<serde_json::Value, crate::error::SdkError>;
    /// ⚠️ 응답 스키마가 미정의(빈 object)되어 raw JSON(`serde_json::Value`)을 반환합니다. PortOne OpenAPI 스펙 한계.
    async fn cancel_platform_contract_schedule(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<serde_json::Value, crate::error::SdkError>;
    /// ⚠️ 응답 스키마가 미정의(빈 object)되어 raw JSON(`serde_json::Value`)을 반환합니다. PortOne OpenAPI 스펙 한계.
    async fn cancel_platform_discount_share_policy_schedule(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<serde_json::Value, crate::error::SdkError>;
    /// ⚠️ 응답 스키마가 미정의(빈 object)되어 raw JSON(`serde_json::Value`)을 반환합니다. PortOne OpenAPI 스펙 한계.
    async fn cancel_platform_partner_schedule(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<serde_json::Value, crate::error::SdkError>;
    async fn complete_platform_payout_by_partner_settlement_ids(
        &self,
        complete_platform_payout_by_partner_settlement_ids_body: models::CompletePlatformPayoutByPartnerSettlementIdsBody,
        test: Option<bool>,
    ) -> Result<models::CompletePlatformPayoutByPartnerSettlementIdsResponse, crate::error::SdkError>;
    async fn create_platform_additional_fee_policy(
        &self,
        create_platform_additional_fee_policy_body: models::CreatePlatformAdditionalFeePolicyBody,
        test: Option<bool>,
    ) -> Result<models::CreatePlatformAdditionalFeePolicyResponse, crate::error::SdkError>;
    async fn create_platform_contract(
        &self,
        create_platform_contract_body: models::CreatePlatformContractBody,
        test: Option<bool>,
    ) -> Result<models::CreatePlatformContractResponse, crate::error::SdkError>;
    async fn create_platform_discount_share_policy(
        &self,
        create_platform_discount_share_policy_body: models::CreatePlatformDiscountSharePolicyBody,
        test: Option<bool>,
    ) -> Result<models::CreatePlatformDiscountSharePolicyResponse, crate::error::SdkError>;
    async fn create_platform_manual_transfer(
        &self,
        create_platform_manual_transfer_body: models::CreatePlatformManualTransferBody,
        test: Option<bool>,
    ) -> Result<models::CreateManualTransferResponse, crate::error::SdkError>;
    async fn create_platform_order_cancel_transfer(
        &self,
        create_platform_order_cancel_transfer_body: models::CreatePlatformOrderCancelTransferBody,
        test: Option<bool>,
    ) -> Result<models::CreateOrderCancelTransferResponse, crate::error::SdkError>;
    async fn create_platform_order_transfer(
        &self,
        create_platform_order_transfer_body: models::CreatePlatformOrderTransferBody,
        test: Option<bool>,
    ) -> Result<models::CreateOrderTransferResponse, crate::error::SdkError>;
    async fn create_platform_partner(
        &self,
        create_platform_partner_body: models::CreatePlatformPartnerBody,
        test: Option<bool>,
    ) -> Result<models::CreatePlatformPartnerResponse, crate::error::SdkError>;
    async fn create_platform_partners(
        &self,
        create_platform_partners_body: models::CreatePlatformPartnersBody,
        test: Option<bool>,
    ) -> Result<models::CreatePlatformPartnersResponse, crate::error::SdkError>;
    /// ⚠️ 응답 스키마가 미정의(빈 object)되어 raw JSON(`serde_json::Value`)을 반환합니다. PortOne OpenAPI 스펙 한계.
    async fn delete_platform_partner_settlements(
        &self,
        delete_platform_partner_settlements_body: models::DeletePlatformPartnerSettlementsBody,
        test: Option<bool>,
    ) -> Result<serde_json::Value, crate::error::SdkError>;
    /// ⚠️ 응답 스키마가 미정의(빈 object)되어 raw JSON(`serde_json::Value`)을 반환합니다. PortOne OpenAPI 스펙 한계.
    async fn delete_platform_transfer(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<serde_json::Value, crate::error::SdkError>;
    async fn download_platform_transfer_sheet(
        &self,
        request_body: models::DownloadPlatformTransferSheetBody,
        test: Option<bool>,
    ) -> Result<reqwest::Response, crate::error::SdkError>;
    async fn get_platform_account_holder(
        &self,
        bank: models::Bank,
        account_number: &str,
        test: Option<bool>,
        birthdate: Option<&str>,
        business_registration_number: Option<&str>,
    ) -> Result<models::PlatformAccountHolder, crate::error::SdkError>;
    async fn get_platform_account_transfers(
        &self,
        test: Option<bool>,
        request_body: Option<models::GetAccountTransfersBody1>,
    ) -> Result<models::GetPlatformAccountTransfersResponse, crate::error::SdkError>;
    async fn get_platform_additional_fee_policies(
        &self,
        test: Option<bool>,
        request_body: Option<models::GetPlatformAdditionalFeePoliciesBody>,
    ) -> Result<models::GetPlatformAdditionalFeePoliciesResponse, crate::error::SdkError>;
    async fn get_platform_additional_fee_policy(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::PlatformAdditionalFeePolicy, crate::error::SdkError>;
    async fn get_platform_additional_fee_policy_schedule(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::PlatformAdditionalFeePolicy, crate::error::SdkError>;
    async fn get_platform_bulk_account_transfers(
        &self,
        test: Option<bool>,
        request_body: Option<models::GetPlatformBulkAccountTransfersBody>,
    ) -> Result<models::GetPlatformBulkAccountTransfersResponse, crate::error::SdkError>;
    async fn get_platform_bulk_payouts(
        &self,
        test: Option<bool>,
        request_body: Option<models::GetPlatformBulkPayoutsBody>,
    ) -> Result<models::GetPlatformBulkPayoutsResponse, crate::error::SdkError>;
    async fn get_platform_company_state(
        &self,
        business_registration_number: &str,
        test: Option<bool>,
    ) -> Result<models::GetPlatformCompanyStatePayload, crate::error::SdkError>;
    async fn get_platform_contract(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::PlatformContract, crate::error::SdkError>;
    async fn get_platform_contract_schedule(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::PlatformContract, crate::error::SdkError>;
    async fn get_platform_contracts(
        &self,
        test: Option<bool>,
        request_body: Option<models::GetPlatformContractsBody>,
    ) -> Result<models::GetPlatformContractsResponse, crate::error::SdkError>;
    async fn get_platform_discount_share_policies(
        &self,
        test: Option<bool>,
        request_body: Option<models::GetPlatformDiscountSharePoliciesBody>,
    ) -> Result<models::GetPlatformDiscountSharePoliciesResponse, crate::error::SdkError>;
    async fn get_platform_discount_share_policy(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::PlatformDiscountSharePolicy, crate::error::SdkError>;
    async fn get_platform_discount_share_policy_filter_options(
        &self,
        test: Option<bool>,
        is_archived: Option<bool>,
    ) -> Result<models::PlatformDiscountSharePolicyFilterOptions, crate::error::SdkError>;
    async fn get_platform_discount_share_policy_schedule(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::PlatformDiscountSharePolicy, crate::error::SdkError>;
    async fn get_platform_partner(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::PlatformPartner, crate::error::SdkError>;
    async fn get_platform_partner_filter_options(
        &self,
        test: Option<bool>,
        is_archived: Option<bool>,
    ) -> Result<models::PlatformPartnerFilterOptions, crate::error::SdkError>;
    async fn get_platform_partner_schedule(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::PlatformPartner, crate::error::SdkError>;
    async fn get_platform_partner_settlements(
        &self,
        test: Option<bool>,
        request_body: Option<models::GetPlatformPartnerSettlementsBody>,
    ) -> Result<models::GetPlatformPartnerSettlementsResponse, crate::error::SdkError>;
    async fn get_platform_partners(
        &self,
        test: Option<bool>,
        request_body: Option<models::GetPlatformPartnersBody>,
    ) -> Result<models::GetPlatformPartnersResponse, crate::error::SdkError>;
    async fn get_platform_payouts(
        &self,
        test: Option<bool>,
        request_body: Option<models::GetPlatformPayoutsBody>,
    ) -> Result<models::GetPlatformPayoutsResponse, crate::error::SdkError>;
    async fn get_platform_setting(
        &self,
        test: Option<bool>,
    ) -> Result<models::PlatformSetting, crate::error::SdkError>;
    async fn get_platform_transfer(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::PlatformTransfer, crate::error::SdkError>;
    async fn get_platform_transfer_summaries(
        &self,
        test: Option<bool>,
        request_body: Option<models::GetPlatformTransferSummariesBody>,
    ) -> Result<models::GetPlatformTransferSummariesResponse, crate::error::SdkError>;
    async fn recover_platform_additional_fee_policy(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::RecoverPlatformAdditionalFeePolicyResponse, crate::error::SdkError>;
    async fn recover_platform_contract(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::RecoverPlatformContractResponse, crate::error::SdkError>;
    async fn recover_platform_discount_share_policy(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::RecoverPlatformDiscountSharePolicyResponse, crate::error::SdkError>;
    async fn recover_platform_partner(
        &self,
        id: &str,
        test: Option<bool>,
    ) -> Result<models::RecoverPlatformPartnerResponse, crate::error::SdkError>;
    /// ⚠️ 응답 스키마가 미정의(빈 object)되어 raw JSON(`serde_json::Value`)을 반환합니다. PortOne OpenAPI 스펙 한계.
    async fn schedule_platform_partners(
        &self,
        schedule_platform_partners_body: models::SchedulePlatformPartnersBody,
        test: Option<bool>,
    ) -> Result<serde_json::Value, crate::error::SdkError>;
    async fn update_platform_additional_fee_policy(
        &self,
        id: &str,
        update_platform_additional_fee_policy_body: models::UpdatePlatformAdditionalFeePolicyBody,
        test: Option<bool>,
    ) -> Result<models::UpdatePlatformAdditionalFeePolicyResponse, crate::error::SdkError>;
    async fn update_platform_contract(
        &self,
        id: &str,
        update_platform_contract_body: models::UpdatePlatformContractBody,
        test: Option<bool>,
    ) -> Result<models::UpdatePlatformContractResponse, crate::error::SdkError>;
    async fn update_platform_discount_share_policy(
        &self,
        id: &str,
        update_platform_discount_share_policy_body: models::UpdatePlatformDiscountSharePolicyBody,
        test: Option<bool>,
    ) -> Result<models::UpdatePlatformDiscountSharePolicyResponse, crate::error::SdkError>;
    async fn update_platform_partner(
        &self,
        id: &str,
        update_platform_partner_body: models::UpdatePlatformPartnerBody,
        test: Option<bool>,
    ) -> Result<models::UpdatePlatformPartnerResponse, crate::error::SdkError>;
    async fn update_platform_setting(
        &self,
        update_platform_setting_body: models::UpdatePlatformSettingBody,
        test: Option<bool>,
    ) -> Result<models::UpdatePlatformSettingResponse, crate::error::SdkError>;
}
