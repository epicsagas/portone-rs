use portone_rs::v2::adapter::generated_api::{Error, ResponseContent};
use portone_rs::v2::SdkError;

#[test]
fn sdk_error_api_error_display() {
    let err = SdkError::ApiError {
        status: reqwest::StatusCode::BAD_REQUEST,
        code: "INVALID_REQUEST".to_string(),
        message: "잘못된 요청입니다".to_string(),
    };

    assert_eq!(
        err.to_string(),
        "API error: 400 Bad Request INVALID_REQUEST - 잘못된 요청입니다"
    );
}

#[test]
fn sdk_error_unknown_display() {
    let err = SdkError::Unknown("boom".to_string());

    assert_eq!(err.to_string(), "Unknown error: boom");
}

#[test]
fn from_generated_response_error_extracts_code_and_message() {
    let generated: Error<()> = Error::ResponseError(ResponseContent {
        status: reqwest::StatusCode::NOT_FOUND,
        content: r#"{"type":"PAYMENT_NOT_FOUND","message":"결제 건을 찾을 수 없습니다"}"#
            .to_string(),
        entity: None,
    });

    let sdk: SdkError = generated.into();

    match sdk {
        SdkError::ApiError {
            status,
            code,
            message,
        } => {
            assert_eq!(status, reqwest::StatusCode::NOT_FOUND);
            assert_eq!(code, "PAYMENT_NOT_FOUND");
            assert_eq!(message, "결제 건을 찾을 수 없습니다");
        }
        other => panic!("expected ApiError, got {other:?}"),
    }
}

#[test]
fn from_generated_response_error_preserves_unknown_body() {
    let generated: Error<()> = Error::ResponseError(ResponseContent {
        status: reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        content: "<html>oops</html>".to_string(),
        entity: None,
    });

    let sdk: SdkError = generated.into();

    match sdk {
        SdkError::ApiError {
            status,
            code,
            message,
        } => {
            assert_eq!(status, reqwest::StatusCode::INTERNAL_SERVER_ERROR);
            assert_eq!(code, "UNKNOWN");
            assert_eq!(message, "<html>oops</html>");
        }
        other => panic!("expected ApiError, got {other:?}"),
    }
}

#[test]
fn from_generated_serde_error_maps_to_serialization() {
    let serde_err = serde_json::from_str::<i32>("not-json").unwrap_err();
    let generated: Error<()> = Error::Serde(serde_err);

    let sdk: SdkError = generated.into();

    assert!(matches!(sdk, SdkError::SerializationError(_)));
}

#[test]
fn from_generated_io_error_maps_to_io() {
    // reqwest::Error는 쉽게 생성할 수 없으므로,
    // GeneratedError::Io → SdkError::Io 매핑을 검증한다 (같은 From 구현 경로).
    let generated: Error<()> = Error::Io(std::io::Error::other("disk full"));

    let sdk: SdkError = generated.into();

    assert!(matches!(sdk, SdkError::Io(_)));
}
