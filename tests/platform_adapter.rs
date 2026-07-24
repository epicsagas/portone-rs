use portone_rs::v2::{HttpClient, PlatformPort};
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn delete_platform_transfer_succeeds() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/platform/transfers/transfer-1"))
        .and(query_param("test", "true"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"deleted": true})),
        )
        .mount(&mock_server)
        .await;

    let client = HttpClient::new("test-secret")
        .expect("Failed to initialize HttpClient")
        .with_base_url(mock_server.uri());

    let result = client
        .delete_platform_transfer("transfer-1", Some(true))
        .await;

    assert_eq!(result.unwrap(), serde_json::json!({"deleted": true}));
}
