use crate::adapter::generated_api::default_api;
use crate::adapter::http_client::HttpClient;
use crate::domain::models;
use crate::port::B2bPort;
use async_trait::async_trait;

#[async_trait]
impl B2bPort for HttpClient {
    async fn attach_b2b_tax_invoice_file(
        &self,
        tax_invoice_key: &str,
        attach_b2b_tax_invoice_file_body: models::AttachB2bTaxInvoiceFileBody,
        brn: Option<&str>,
        tax_invoice_key_type: Option<models::B2bTaxInvoiceKeyType>,
        test: Option<bool>,
    ) -> Result<(), crate::error::SdkError> {
        default_api::attach_b2b_tax_invoice_file(
            &self.configuration,
            tax_invoice_key,
            attach_b2b_tax_invoice_file_body,
            brn,
            tax_invoice_key_type,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn cancel_b2b_tax_invoice_issuance(
        &self,
        tax_invoice_key: &str,
        cancel_b2b_tax_invoice_issuance_body: models::CancelB2bTaxInvoiceIssuanceBody,
        brn: Option<&str>,
        tax_invoice_key_type: Option<models::B2bTaxInvoiceKeyType>,
        test: Option<bool>,
    ) -> Result<models::CancelB2bTaxInvoiceIssuanceResponse, crate::error::SdkError> {
        default_api::cancel_b2b_tax_invoice_issuance(
            &self.configuration,
            tax_invoice_key,
            cancel_b2b_tax_invoice_issuance_body,
            brn,
            tax_invoice_key_type,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn cancel_b2b_tax_invoice_request(
        &self,
        tax_invoice_key: &str,
        cancel_b2b_tax_invoice_request_body: models::CancelB2bTaxInvoiceRequestBody,
        brn: Option<&str>,
        tax_invoice_key_type: Option<models::B2bTaxInvoiceKeyType>,
        test: Option<bool>,
    ) -> Result<models::CancelB2bTaxInvoiceRequestResponse, crate::error::SdkError> {
        default_api::cancel_b2b_tax_invoice_request(
            &self.configuration,
            tax_invoice_key,
            cancel_b2b_tax_invoice_request_body,
            brn,
            tax_invoice_key_type,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn create_b2b_counterparty(
        &self,
        create_b2b_counterparty_body: models::CreateB2bCounterpartyBody,
        test: Option<bool>,
    ) -> Result<models::CreateB2bCounterpartyResponse, crate::error::SdkError> {
        default_api::create_b2b_counterparty(
            &self.configuration,
            create_b2b_counterparty_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn create_b2b_file_upload_url(
        &self,
        create_b2b_file_upload_url_body: models::CreateB2bFileUploadUrlBody,
        test: Option<bool>,
    ) -> Result<models::CreateB2bFileUploadUrlPayload, crate::error::SdkError> {
        default_api::create_b2b_file_upload_url(
            &self.configuration,
            create_b2b_file_upload_url_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn delete_b2b_counterparty(
        &self,
        counterparty_id: &str,
        test: Option<bool>,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, crate::error::SdkError> {
        default_api::delete_b2b_counterparty(&self.configuration, counterparty_id, test, body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn delete_b2b_tax_invoice(
        &self,
        tax_invoice_key: &str,
        brn: Option<&str>,
        tax_invoice_key_type: Option<models::B2bTaxInvoiceKeyType>,
        test: Option<bool>,
    ) -> Result<serde_json::Value, crate::error::SdkError> {
        default_api::delete_b2b_tax_invoice(
            &self.configuration,
            tax_invoice_key,
            brn,
            tax_invoice_key_type,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn delete_b2b_tax_invoice_attachment(
        &self,
        tax_invoice_key: &str,
        attachment_id: &str,
        brn: Option<&str>,
        tax_invoice_key_type: Option<models::B2bTaxInvoiceKeyType>,
        test: Option<bool>,
    ) -> Result<(), crate::error::SdkError> {
        default_api::delete_b2b_tax_invoice_attachment(
            &self.configuration,
            tax_invoice_key,
            attachment_id,
            brn,
            tax_invoice_key_type,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn download_b2b_tax_invoices_sheet(
        &self,
        request_body: models::DownloadB2bTaxInvoicesSheetBody,
    ) -> Result<reqwest::Response, crate::error::SdkError> {
        default_api::download_b2b_tax_invoices_sheet(&self.configuration, request_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn draft_b2b_tax_invoice(
        &self,
        draft_b2b_tax_invoice_body: models::DraftB2bTaxInvoiceBody,
        test: Option<bool>,
    ) -> Result<models::DraftB2bTaxInvoiceResponse, crate::error::SdkError> {
        default_api::draft_b2b_tax_invoice(&self.configuration, draft_b2b_tax_invoice_body, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_b2b_bulk_tax_invoice(
        &self,
        bulk_tax_invoice_id: &str,
        test: Option<bool>,
    ) -> Result<models::B2bBulkTaxInvoice, crate::error::SdkError> {
        default_api::get_b2b_bulk_tax_invoice(&self.configuration, bulk_tax_invoice_id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_b2b_business_infos(
        &self,
        get_b2b_business_infos_body: models::GetB2bBusinessInfosBody,
        test: Option<bool>,
    ) -> Result<models::GetB2bBusinessInfosResponse, crate::error::SdkError> {
        default_api::get_b2b_business_infos(&self.configuration, get_b2b_business_infos_body, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_b2b_counterparties(
        &self,
        test: Option<bool>,
        request_body: Option<models::GetB2bCounterpartiesBody>,
    ) -> Result<models::GetB2bCounterpartiesResponse, crate::error::SdkError> {
        default_api::get_b2b_counterparties(&self.configuration, test, request_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_b2b_counterparty(
        &self,
        counterparty_id: &str,
        test: Option<bool>,
    ) -> Result<models::B2bCounterparty, crate::error::SdkError> {
        default_api::get_b2b_counterparty(&self.configuration, counterparty_id, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_b2b_counterparty_certificate(
        &self,
        brn: &str,
        test: Option<bool>,
    ) -> Result<models::B2bCertificate, crate::error::SdkError> {
        default_api::get_b2b_counterparty_certificate(&self.configuration, brn, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_b2b_counterparty_certificate_registration_url(
        &self,
        brn: &str,
        test: Option<bool>,
    ) -> Result<models::GetB2bCounterpartyCertificateRegistrationUrlResponse, crate::error::SdkError>
    {
        default_api::get_b2b_counterparty_certificate_registration_url(
            &self.configuration,
            brn,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn get_b2b_tax_invoice(
        &self,
        tax_invoice_key: &str,
        brn: Option<&str>,
        tax_invoice_key_type: Option<models::B2bTaxInvoiceKeyType>,
        test: Option<bool>,
    ) -> Result<models::B2bTaxInvoice, crate::error::SdkError> {
        default_api::get_b2b_tax_invoice(
            &self.configuration,
            tax_invoice_key,
            brn,
            tax_invoice_key_type,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn get_b2b_tax_invoice_attachments(
        &self,
        tax_invoice_key: &str,
        brn: Option<&str>,
        tax_invoice_key_type: Option<models::B2bTaxInvoiceKeyType>,
        test: Option<bool>,
    ) -> Result<models::GetB2bTaxInvoiceAttachmentsResponse, crate::error::SdkError> {
        default_api::get_b2b_tax_invoice_attachments(
            &self.configuration,
            tax_invoice_key,
            brn,
            tax_invoice_key_type,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn get_b2b_tax_invoice_pdf_download_url(
        &self,
        tax_invoice_key: &str,
        brn: Option<&str>,
        tax_invoice_key_type: Option<models::B2bTaxInvoiceKeyType>,
        test: Option<bool>,
    ) -> Result<models::GetB2bTaxInvoicePdfDownloadUrlResponse, crate::error::SdkError> {
        default_api::get_b2b_tax_invoice_pdf_download_url(
            &self.configuration,
            tax_invoice_key,
            brn,
            tax_invoice_key_type,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn get_b2b_tax_invoice_popup_url(
        &self,
        tax_invoice_key: &str,
        brn: Option<&str>,
        tax_invoice_key_type: Option<models::B2bTaxInvoiceKeyType>,
        include_menu: Option<bool>,
        test: Option<bool>,
    ) -> Result<models::GetB2bTaxInvoicePopupUrlResponse, crate::error::SdkError> {
        default_api::get_b2b_tax_invoice_popup_url(
            &self.configuration,
            tax_invoice_key,
            brn,
            tax_invoice_key_type,
            include_menu,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn get_b2b_tax_invoice_print_url(
        &self,
        tax_invoice_key: &str,
        brn: Option<&str>,
        tax_invoice_key_type: Option<models::B2bTaxInvoiceKeyType>,
        test: Option<bool>,
    ) -> Result<models::GetB2bTaxInvoicePrintUrlResponse, crate::error::SdkError> {
        default_api::get_b2b_tax_invoice_print_url(
            &self.configuration,
            tax_invoice_key,
            brn,
            tax_invoice_key_type,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn get_b2b_tax_invoices(
        &self,
        request_body: Option<models::GetB2bTaxInvoicesBody>,
    ) -> Result<models::GetB2bTaxInvoicesResponse, crate::error::SdkError> {
        default_api::get_b2b_tax_invoices(&self.configuration, request_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn issue_b2b_tax_invoice(
        &self,
        tax_invoice_key: &str,
        issue_b2b_tax_invoice_body: models::IssueB2bTaxInvoiceBody,
        brn: Option<&str>,
        tax_invoice_key_type: Option<models::B2bTaxInvoiceKeyType>,
        test: Option<bool>,
    ) -> Result<models::IssueB2bTaxInvoiceResponse, crate::error::SdkError> {
        default_api::issue_b2b_tax_invoice(
            &self.configuration,
            tax_invoice_key,
            issue_b2b_tax_invoice_body,
            brn,
            tax_invoice_key_type,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn issue_b2b_tax_invoice_immediately(
        &self,
        issue_b2b_tax_invoice_immediately_body: models::IssueB2bTaxInvoiceImmediatelyBody,
        test: Option<bool>,
    ) -> Result<models::IssueB2bTaxInvoiceImmediatelyResponse, crate::error::SdkError> {
        default_api::issue_b2b_tax_invoice_immediately(
            &self.configuration,
            issue_b2b_tax_invoice_immediately_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn refuse_b2b_tax_invoice_request(
        &self,
        tax_invoice_key: &str,
        refuse_b2b_tax_invoice_request_body: models::RefuseB2bTaxInvoiceRequestBody,
        brn: Option<&str>,
        tax_invoice_key_type: Option<models::B2bTaxInvoiceKeyType>,
        test: Option<bool>,
    ) -> Result<models::RefuseB2bTaxInvoiceRequestResponse, crate::error::SdkError> {
        default_api::refuse_b2b_tax_invoice_request(
            &self.configuration,
            tax_invoice_key,
            refuse_b2b_tax_invoice_request_body,
            brn,
            tax_invoice_key_type,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn request_b2b_tax_invoice(
        &self,
        tax_invoice_key: &str,
        brn: Option<&str>,
        tax_invoice_key_type: Option<models::B2bTaxInvoiceKeyType>,
        test: Option<bool>,
    ) -> Result<models::RequestB2bTaxInvoiceResponse, crate::error::SdkError> {
        default_api::request_b2b_tax_invoice(
            &self.configuration,
            tax_invoice_key,
            brn,
            tax_invoice_key_type,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn request_b2b_tax_invoice_reverse_issuance(
        &self,
        request_b2b_tax_invoice_reverse_issuance_body: models::RequestB2bTaxInvoiceReverseIssuanceBody,
        test: Option<bool>,
    ) -> Result<models::RequestB2bTaxInvoiceReverseIssuanceResponse, crate::error::SdkError> {
        default_api::request_b2b_tax_invoice_reverse_issuance(
            &self.configuration,
            request_b2b_tax_invoice_reverse_issuance_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn send_to_nts_b2b_tax_invoice(
        &self,
        tax_invoice_key: &str,
        brn: Option<&str>,
        tax_invoice_key_type: Option<models::B2bTaxInvoiceKeyType>,
        test: Option<bool>,
    ) -> Result<models::SendToNtsB2bTaxInvoiceResponse, crate::error::SdkError> {
        default_api::send_to_nts_b2b_tax_invoice(
            &self.configuration,
            tax_invoice_key,
            brn,
            tax_invoice_key_type,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn update_b2b_counterparty(
        &self,
        counterparty_id: &str,
        update_b2b_counterparty_body: models::UpdateB2bCounterpartyBody,
        test: Option<bool>,
    ) -> Result<models::UpdateB2bCounterpartyResponse, crate::error::SdkError> {
        default_api::update_b2b_counterparty(
            &self.configuration,
            counterparty_id,
            update_b2b_counterparty_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn update_b2b_tax_invoice_draft(
        &self,
        update_b2b_tax_invoice_draft_body: models::UpdateB2bTaxInvoiceDraftBody,
        test: Option<bool>,
    ) -> Result<models::UpdateB2bTaxInvoiceDraftResponse, crate::error::SdkError> {
        default_api::update_b2b_tax_invoice_draft(
            &self.configuration,
            update_b2b_tax_invoice_draft_body,
            test,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn validate_b2b_counterparty_certificate(
        &self,
        brn: &str,
        test: Option<bool>,
    ) -> Result<models::ValidateB2bCounterpartyCertificateResponse, crate::error::SdkError> {
        default_api::validate_b2b_counterparty_certificate(&self.configuration, brn, test)
            .await
            .map_err(crate::error::SdkError::from)
    }
}
