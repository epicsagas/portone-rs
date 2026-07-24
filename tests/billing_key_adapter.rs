use portone_rs::v2::{BillingKeyPort, HttpClient, SdkError};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn client(base_url: String) -> HttpClient {
    HttpClient::new("test-secret")
        .expect("Failed to initialize HttpClient")
        .with_base_url(base_url)
}

#[tokio::test]
async fn delete_billing_key_succeeds() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/billing-keys/billing-key-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "deletedAt": "2026-07-21T00:00:00+09:00"
        })))
        .mount(&mock_server)
        .await;

    let response = client(mock_server.uri())
        .delete_billing_key("billing-key-1", None, None, None, None)
        .await
        .expect("expected successful response");

    let expected = chrono::DateTime::parse_from_rfc3339("2026-07-21T00:00:00+09:00").unwrap();
    assert_eq!(response.deleted_at, expected);
}

#[tokio::test]
async fn delete_billing_key_unauthorized_returns_response_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/billing-keys/billing-key-1"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "type": "UNAUTHORIZED",
            "message": "인증되지 않은 요청입니다"
        })))
        .mount(&mock_server)
        .await;

    let result = client(mock_server.uri())
        .delete_billing_key("billing-key-1", None, None, None, None)
        .await;

    match result {
        Err(SdkError::ApiError { status, code, .. }) => {
            assert_eq!(status, reqwest::StatusCode::UNAUTHORIZED);
            assert_eq!(code, "UNAUTHORIZED");
        }
        other => panic!("expected ApiError, got {other:?}"),
    }
}
