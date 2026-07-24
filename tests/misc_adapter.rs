use portone_rs::v2::{HttpClient, MiscPort, SdkError};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn client(base_url: String) -> HttpClient {
    HttpClient::new("test-secret")
        .expect("Failed to initialize HttpClient")
        .with_base_url(base_url)
}

#[tokio::test]
async fn get_bank_infos_succeeds() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/banks"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "items": [
                { "bank": "KOOKMIN", "name": { "ko": "국민은행" } }
            ]
        })))
        .mount(&mock_server)
        .await;

    let response = client(mock_server.uri())
        .get_bank_infos()
        .await
        .expect("expected successful response");

    assert_eq!(response.items.len(), 1);
    assert_eq!(response.items[0].name.ko, "국민은행");
}

#[tokio::test]
async fn get_bank_infos_unexpected_content_type_is_serde_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/banks"))
        .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
        .mount(&mock_server)
        .await;

    let result = client(mock_server.uri()).get_bank_infos().await;

    assert!(matches!(result, Err(SdkError::SerializationError(_))));
}
