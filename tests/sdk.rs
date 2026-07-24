use portone_rs::v2::HttpClient;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn test_client_initialization() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let _client = HttpClient::new("YOUR_PORTONE_API_SECRET")
        .expect("Failed to initialize HttpClient")
        .with_base_url(mock_server.uri());
}
