use crate::adapter::generated_api::default_api;
use crate::adapter::http_client::HttpClient;
use crate::domain::models;
use crate::port::PaymentPort;
use async_trait::async_trait;

#[async_trait]
impl PaymentPort for HttpClient {
    async fn cancel_cash_receipt_by_payment_id(
        &self,
        payment_id: &str,
        store_id: Option<&str>,
    ) -> Result<models::CancelCashReceiptResponse, crate::error::SdkError> {
        default_api::cancel_cash_receipt_by_payment_id(&self.configuration, payment_id, store_id)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn cancel_payment(
        &self,
        payment_id: &str,
        cancel_payment_body: models::CancelPaymentBody,
    ) -> Result<models::CancelPaymentResponse, crate::error::SdkError> {
        default_api::cancel_payment(&self.configuration, payment_id, cancel_payment_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn capture_payment(
        &self,
        payment_id: &str,
        capture_payment_body: models::CapturePaymentBody,
    ) -> Result<serde_json::Value, crate::error::SdkError> {
        default_api::capture_payment(&self.configuration, payment_id, capture_payment_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn close_payment_session(
        &self,
        session_id: &str,
    ) -> Result<models::ClosePaymentSessionResponse, crate::error::SdkError> {
        default_api::close_payment_session(&self.configuration, session_id)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn confirm_payment(
        &self,
        payment_id: &str,
        confirm_payment_body: models::ConfirmPaymentBody,
    ) -> Result<models::ConfirmedPaymentSummary, crate::error::SdkError> {
        default_api::confirm_payment(&self.configuration, payment_id, confirm_payment_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn confirm_paymentwall_delivery(
        &self,
        confirm_paymentwall_delivery_body: models::ConfirmPaymentwallDeliveryBody,
    ) -> Result<serde_json::Value, crate::error::SdkError> {
        default_api::confirm_paymentwall_delivery(
            &self.configuration,
            confirm_paymentwall_delivery_body,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn create_payment_schedule(
        &self,
        payment_id: &str,
        create_payment_schedule_body: models::CreatePaymentScheduleBody,
    ) -> Result<models::CreatePaymentScheduleResponse, crate::error::SdkError> {
        default_api::create_payment_schedule(
            &self.configuration,
            payment_id,
            create_payment_schedule_body,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn create_payment_session(
        &self,
        create_payment_session_body: models::CreatePaymentSessionBody,
    ) -> Result<models::CreatePaymentSessionResponse, crate::error::SdkError> {
        default_api::create_payment_session(&self.configuration, create_payment_session_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_all_payment_events_by_cursor(
        &self,
        request_body: Option<models::GetAllPaymentEventsByCursorBody>,
    ) -> Result<models::GetAllPaymentEventsByCursorResponse, crate::error::SdkError> {
        default_api::get_all_payment_events_by_cursor(&self.configuration, request_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_all_payments_by_cursor(
        &self,
        request_body: Option<models::GetAllPaymentsByCursorBody>,
    ) -> Result<models::GetAllPaymentsByCursorResponse, crate::error::SdkError> {
        default_api::get_all_payments_by_cursor(&self.configuration, request_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_cash_receipt_by_payment_id(
        &self,
        payment_id: &str,
        store_id: Option<&str>,
    ) -> Result<models::CashReceipt, crate::error::SdkError> {
        default_api::get_cash_receipt_by_payment_id(&self.configuration, payment_id, store_id)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_kakaopay_payment_order(
        &self,
        pg_tx_id: &str,
        channel_key: &str,
    ) -> Result<models::GetKakaopayPaymentOrderResponse, crate::error::SdkError> {
        default_api::get_kakaopay_payment_order(&self.configuration, pg_tx_id, channel_key)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_payment(
        &self,
        payment_id: &str,
        store_id: Option<&str>,
    ) -> Result<models::Payment, crate::error::SdkError> {
        default_api::get_payment(&self.configuration, payment_id, store_id)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_payment_reconciliation_settlement_vat_report(
        &self,
        request_body: models::GetPaymentReconciliationSettlementVatReportBody,
    ) -> Result<models::GetPaymentReconciliationSettlementVatReportResponse, crate::error::SdkError>
    {
        default_api::get_payment_reconciliation_settlement_vat_report(
            &self.configuration,
            request_body,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn get_payment_reconciliation_transaction_vat_report(
        &self,
        request_body: models::GetPaymentReconciliationTransactionVatReportBody,
    ) -> Result<models::GetPaymentReconciliationTransactionVatReportResponse, crate::error::SdkError>
    {
        default_api::get_payment_reconciliation_transaction_vat_report(
            &self.configuration,
            request_body,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn get_payment_schedule(
        &self,
        payment_schedule_id: &str,
        store_id: Option<&str>,
    ) -> Result<models::PaymentSchedule, crate::error::SdkError> {
        default_api::get_payment_schedule(&self.configuration, payment_schedule_id, store_id)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_payment_schedules(
        &self,
        request_body: Option<models::GetPaymentSchedulesBody>,
    ) -> Result<models::GetPaymentSchedulesResponse, crate::error::SdkError> {
        default_api::get_payment_schedules(&self.configuration, request_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_payment_session(
        &self,
        session_id: &str,
    ) -> Result<models::PaymentSession, crate::error::SdkError> {
        default_api::get_payment_session(&self.configuration, session_id)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_payment_transactions(
        &self,
        payment_id: &str,
        store_id: Option<&str>,
    ) -> Result<models::GetPaymentTransactionsResponse, crate::error::SdkError> {
        default_api::get_payment_transactions(&self.configuration, payment_id, store_id)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn get_payments(
        &self,
        request_body: Option<models::GetPaymentsBody>,
    ) -> Result<models::GetPaymentsResponse, crate::error::SdkError> {
        default_api::get_payments(&self.configuration, request_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn pre_register_payment(
        &self,
        payment_id: &str,
        pre_register_payment_body: models::PreRegisterPaymentBody,
    ) -> Result<serde_json::Value, crate::error::SdkError> {
        default_api::pre_register_payment(
            &self.configuration,
            payment_id,
            pre_register_payment_body,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
    async fn revoke_payment_schedules(
        &self,
        request_body: models::RevokePaymentSchedulesBody,
    ) -> Result<models::RevokePaymentSchedulesResponse, crate::error::SdkError> {
        default_api::revoke_payment_schedules(&self.configuration, request_body)
            .await
            .map_err(crate::error::SdkError::from)
    }
    async fn stop_payment_cancellation(
        &self,
        payment_id: &str,
        cancellation_id: &str,
        stop_payment_cancellation_body: models::StopPaymentCancellationBody,
    ) -> Result<models::StopPaymentCancellationResponse, crate::error::SdkError> {
        default_api::stop_payment_cancellation(
            &self.configuration,
            payment_id,
            cancellation_id,
            stop_payment_cancellation_body,
        )
        .await
        .map_err(crate::error::SdkError::from)
    }
}
