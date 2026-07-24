use portone_rs::v2::domain::models;
use portone_rs::v2::{B2bPort, HttpClient, SdkError};
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn client(base_url: String) -> HttpClient {
    HttpClient::new("test-secret")
        .expect("Failed to initialize HttpClient")
        .with_base_url(base_url)
}

#[tokio::test]
async fn attach_b2b_tax_invoice_file_succeeds() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/b2b/tax-invoices/tax-invoice-key-1/attach-file"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let result = client(mock_server.uri())
        .attach_b2b_tax_invoice_file(
            "tax-invoice-key-1",
            models::AttachB2bTaxInvoiceFileBody::new("file-1".to_string()),
            None,
            None,
            None,
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn attach_b2b_tax_invoice_file_returns_response_error_on_404() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/b2b/tax-invoices/missing-key/attach-file"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "type": "B2B_TAX_INVOICE_NOT_FOUND",
            "message": "세금계산서를 찾을 수 없습니다"
        })))
        .mount(&mock_server)
        .await;

    let result = client(mock_server.uri())
        .attach_b2b_tax_invoice_file(
            "missing-key",
            models::AttachB2bTaxInvoiceFileBody::new("file-1".to_string()),
            None,
            None,
            None,
        )
        .await;

    match result {
        Err(SdkError::ApiError {
            status,
            code,
            message,
        }) => {
            assert_eq!(status, reqwest::StatusCode::NOT_FOUND);
            assert_eq!(code, "B2B_TAX_INVOICE_NOT_FOUND");
            assert_eq!(message, "세금계산서를 찾을 수 없습니다");
        }
        other => panic!("expected ApiError, got {other:?}"),
    }
}

#[tokio::test]
async fn delete_b2b_counterparty_succeeds() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/b2b/counterparties/counterparty-1"))
        .and(query_param("test", "true"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"ok": true})))
        .mount(&mock_server)
        .await;

    let result = client(mock_server.uri())
        .delete_b2b_counterparty("counterparty-1", Some(true), None)
        .await;

    assert_eq!(result.unwrap(), serde_json::json!({"ok": true}));
}
