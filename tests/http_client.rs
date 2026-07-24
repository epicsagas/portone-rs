use portone_rs::v2::{HttpClient, MiscPort, SdkError};
use wiremock::{
    matchers::{header, method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn new_sets_bearer_authorization_header() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/banks"))
        .and(header("authorization", "PortOne test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({ "items": [] })))
        .mount(&mock_server)
        .await;

    let client = HttpClient::new("test-secret")
        .expect("Failed to initialize HttpClient")
        .with_base_url(mock_server.uri());

    let result = client.get_bank_infos().await;

    assert!(
        result.is_ok(),
        "expected request to carry the configured Authorization header, got {result:?}"
    );
}

#[test]
fn new_rejects_invalid_header_characters() {
    let result = HttpClient::new("invalid\nsecret");

    assert!(matches!(result, Err(SdkError::InvalidHeaderValue(_))));
}
