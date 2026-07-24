//! # trait 기반 모킹으로 HTTP 없이 단위 테스트하기
//!
//! 이 SDK의 도메인 trait(`PaymentPort` 등) 덕분에, 애플리케이션은 구체적 타입
//! (`HttpClient`)이 아닌 trait에 의존하도록 설계할 수 있습니다. 그러면 테스트에서
//! 가짜 구현체를 직접 만들어 네트워크나 mock 서버 없이 비즈니스 로직을 검증할 수 있습니다.
//!
//! > **참고 — 왜 mockall 대신 자체 fake인가:** `PaymentPort`는 `async_trait` + 참조 인자
//! > (`&str`, `Option<&str>`)를 함께 쓰는데, 이 조합에서 `mockall::mock!`는 lifetime을
//! > 처리하지 못해 컴파일이 깨집니다. trait가 클 때는 가벼운 자체 fake가 훨씬 실용적입니다.
//! > 아래 예제는 그 패턴을 `PaymentPort`로 보여줍니다.

use async_trait::async_trait;
use portone_rs::prelude::*; // SdkError, PaymentPort, ...
use portone_rs::v2::domain::models;

/// HTTP 호출 없이 미리 정해둔 동작을 하는 테스트 전용 결제 클라이언트.
struct FakePaymentPort {
    /// "결제건 없음"으로 취급할 payment_id 목록.
    missing_ids: Vec<String>,
}

impl FakePaymentPort {
    fn new(missing_ids: Vec<&str>) -> Self {
        Self {
            missing_ids: missing_ids.into_iter().map(String::from).collect(),
        }
    }
}

#[async_trait]
impl PaymentPort for FakePaymentPort {
    async fn get_payment(
        &self,
        payment_id: &str,
        _store_id: Option<&str>,
    ) -> Result<models::Payment, SdkError> {
        if self.missing_ids.iter().any(|id| id == payment_id) {
            Err(SdkError::Unknown(
                "mock: 결제건을 찾을 수 없습니다".to_string(),
            ))
        } else {
            // 실제 테스트에서는 더미 Payment 객체를 반환한다.
            // (이 예제에서는 모델 생성을 생략하고 동일한 에러 경로를 둔다.)
            Err(SdkError::Unknown(
                "mock: 결제건을 찾을 수 없습니다".to_string(),
            ))
        }
    }

    // 이 예제는 get_payment만 사용하므로 나머지 엔드포인트는 기본 stub으로 둔다.
    // 실제 테스트에서는 필요한 메서드만 의미 있는 값으로 채운다.
    async fn cancel_cash_receipt_by_payment_id(
        &self,
        _: &str,
        _: Option<&str>,
    ) -> Result<models::CancelCashReceiptResponse, SdkError> {
        Err(SdkError::Unknown("stub".to_string()))
    }
    async fn cancel_payment(
        &self,
        _: &str,
        _: models::CancelPaymentBody,
    ) -> Result<models::CancelPaymentResponse, SdkError> {
        Err(SdkError::Unknown("stub".to_string()))
    }
    async fn capture_payment(
        &self,
        _: &str,
        _: models::CapturePaymentBody,
    ) -> Result<serde_json::Value, SdkError> {
        Err(SdkError::Unknown("stub".to_string()))
    }
    async fn close_payment_session(
        &self,
        _: &str,
    ) -> Result<models::ClosePaymentSessionResponse, SdkError> {
        Err(SdkError::Unknown("stub".to_string()))
    }
    async fn confirm_payment(
        &self,
        _: &str,
        _: models::ConfirmPaymentBody,
    ) -> Result<models::ConfirmedPaymentSummary, SdkError> {
        Err(SdkError::Unknown("stub".to_string()))
    }
    async fn confirm_paymentwall_delivery(
        &self,
        _: models::ConfirmPaymentwallDeliveryBody,
    ) -> Result<serde_json::Value, SdkError> {
        Err(SdkError::Unknown("stub".to_string()))
    }
    async fn create_payment_schedule(
        &self,
        _: &str,
        _: models::CreatePaymentScheduleBody,
    ) -> Result<models::CreatePaymentScheduleResponse, SdkError> {
        Err(SdkError::Unknown("stub".to_string()))
    }
    async fn create_payment_session(
        &self,
        _: models::CreatePaymentSessionBody,
    ) -> Result<models::CreatePaymentSessionResponse, SdkError> {
        Err(SdkError::Unknown("stub".to_string()))
    }
    async fn get_all_payment_events_by_cursor(
        &self,
        _: Option<models::GetAllPaymentEventsByCursorBody>,
    ) -> Result<models::GetAllPaymentEventsByCursorResponse, SdkError> {
        Err(SdkError::Unknown("stub".to_string()))
    }
    async fn get_all_payments_by_cursor(
        &self,
        _: Option<models::GetAllPaymentsByCursorBody>,
    ) -> Result<models::GetAllPaymentsByCursorResponse, SdkError> {
        Err(SdkError::Unknown("stub".to_string()))
    }
    async fn get_cash_receipt_by_payment_id(
        &self,
        _: &str,
        _: Option<&str>,
    ) -> Result<models::CashReceipt, SdkError> {
        Err(SdkError::Unknown("stub".to_string()))
    }
    async fn get_kakaopay_payment_order(
        &self,
        _: &str,
        _: &str,
    ) -> Result<models::GetKakaopayPaymentOrderResponse, SdkError> {
        Err(SdkError::Unknown("stub".to_string()))
    }
    async fn get_payment_reconciliation_settlement_vat_report(
        &self,
        _: models::GetPaymentReconciliationSettlementVatReportBody,
    ) -> Result<models::GetPaymentReconciliationSettlementVatReportResponse, SdkError> {
        Err(SdkError::Unknown("stub".to_string()))
    }
    async fn get_payment_reconciliation_transaction_vat_report(
        &self,
        _: models::GetPaymentReconciliationTransactionVatReportBody,
    ) -> Result<models::GetPaymentReconciliationTransactionVatReportResponse, SdkError> {
        Err(SdkError::Unknown("stub".to_string()))
    }
    async fn get_payment_schedule(
        &self,
        _: &str,
        _: Option<&str>,
    ) -> Result<models::PaymentSchedule, SdkError> {
        Err(SdkError::Unknown("stub".to_string()))
    }
    async fn get_payment_schedules(
        &self,
        _: Option<models::GetPaymentSchedulesBody>,
    ) -> Result<models::GetPaymentSchedulesResponse, SdkError> {
        Err(SdkError::Unknown("stub".to_string()))
    }
    async fn get_payment_session(&self, _: &str) -> Result<models::PaymentSession, SdkError> {
        Err(SdkError::Unknown("stub".to_string()))
    }
    async fn get_payment_transactions(
        &self,
        _: &str,
        _: Option<&str>,
    ) -> Result<models::GetPaymentTransactionsResponse, SdkError> {
        Err(SdkError::Unknown("stub".to_string()))
    }
    async fn get_payments(
        &self,
        _: Option<models::GetPaymentsBody>,
    ) -> Result<models::GetPaymentsResponse, SdkError> {
        Err(SdkError::Unknown("stub".to_string()))
    }
    async fn pre_register_payment(
        &self,
        _: &str,
        _: models::PreRegisterPaymentBody,
    ) -> Result<serde_json::Value, SdkError> {
        Err(SdkError::Unknown("stub".to_string()))
    }
    async fn revoke_payment_schedules(
        &self,
        _: models::RevokePaymentSchedulesBody,
    ) -> Result<models::RevokePaymentSchedulesResponse, SdkError> {
        Err(SdkError::Unknown("stub".to_string()))
    }
    async fn stop_payment_cancellation(
        &self,
        _: &str,
        _: &str,
        _: models::StopPaymentCancellationBody,
    ) -> Result<models::StopPaymentCancellationResponse, SdkError> {
        Err(SdkError::Unknown("stub".to_string()))
    }
}

/// 비즈니스 로직은 구체적 타입이 아닌 `PaymentPort` trait에 의존한다.
/// 따라서 `HttpClient`(운영)와 `FakePaymentPort`(테스트) 모두 주입할 수 있다.
async fn is_payment_missing(client: &impl PaymentPort, payment_id: &str) -> bool {
    matches!(
        client.get_payment(payment_id, None).await,
        Err(SdkError::Unknown(_))
    )
}

#[tokio::main]
async fn main() {
    // 운영에서는 HttpClient::new(secret)을 주입하지만, 여기서는 HTTP 없이 fake를 쓴다.
    let fake = FakePaymentPort::new(vec!["ghost-payment"]);

    let missing = is_payment_missing(&fake, "ghost-payment").await;
    println!("'ghost-payment' 누락 여부 = {missing}");
    assert!(missing);
}
