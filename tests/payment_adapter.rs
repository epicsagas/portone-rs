use portone_rs::v2::domain::models;
use portone_rs::v2::{HttpClient, PaymentPort, SdkError};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn client(base_url: String) -> HttpClient {
    HttpClient::new("test-secret")
        .expect("Failed to initialize HttpClient")
        .with_base_url(base_url)
}

#[tokio::test]
async fn capture_payment_succeeds() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/payments/payment-1/capture"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"captured": true})),
        )
        .mount(&mock_server)
        .await;

    let result = client(mock_server.uri())
        .capture_payment("payment-1", models::CapturePaymentBody::new())
        .await;

    assert_eq!(result.unwrap(), serde_json::json!({"captured": true}));
}

#[tokio::test]
async fn capture_payment_not_found_returns_response_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/payments/missing-payment/capture"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "type": "PAYMENT_NOT_FOUND",
            "message": "결제 건을 찾을 수 없습니다"
        })))
        .mount(&mock_server)
        .await;

    let result = client(mock_server.uri())
        .capture_payment("missing-payment", models::CapturePaymentBody::new())
        .await;

    match result {
        Err(SdkError::ApiError { status, code, .. }) => {
            assert_eq!(status, reqwest::StatusCode::NOT_FOUND);
            assert_eq!(code, "PAYMENT_NOT_FOUND");
        }
        other => panic!("expected ApiError, got {other:?}"),
    }
}
