use portone_rs::v2::{HttpClient, IdentityVerificationPort};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn resend_identity_verification_succeeds() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(
            "/identity-verifications/identity-verification-1/resend",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"resent": true})))
        .mount(&mock_server)
        .await;

    let client = HttpClient::new("test-secret")
        .expect("Failed to initialize HttpClient")
        .with_base_url(mock_server.uri());

    let result = client
        .resend_identity_verification("identity-verification-1", None)
        .await;

    assert_eq!(result.unwrap(), serde_json::json!({"resent": true}));
}
