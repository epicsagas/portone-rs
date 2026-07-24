use crate::adapter::generated_api::{Error as GeneratedError, ResponseContent};
use thiserror::Error;

/// SDK에서 발생할 수 있는 단일 에러 타입입니다.
///
/// `HttpClient::new` 초기화와 모든 API 호출이 공통으로 반환하므로,
/// 호출자는 하나의 에러 타입만 다루면 됩니다.
#[derive(Error, Debug)]
pub enum SdkError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("API error: {status} {code} - {message}")]
    ApiError {
        status: reqwest::StatusCode,
        code: String,
        message: String,
    },
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl<T> From<GeneratedError<T>> for SdkError {
    fn from(e: GeneratedError<T>) -> Self {
        match e {
            GeneratedError::Reqwest(e) => SdkError::HttpError(e),
            GeneratedError::Serde(e) => SdkError::SerializationError(e),
            GeneratedError::Io(e) => SdkError::Io(e),
            GeneratedError::ResponseError(rc) => response_to_sdk_error(rc),
        }
    }
}

fn response_to_sdk_error<T>(rc: ResponseContent<T>) -> SdkError {
    let (code, message) = parse_error_body(&rc.content);
    SdkError::ApiError {
        status: rc.status,
        code,
        message,
    }
}

/// 응답 본문에서 `type`/`message` 필드를 추출합니다.
/// JSON이 아니거나 필드가 없으면 원본 content를 보존합니다(정보 손실 없음).
fn parse_error_body(content: &str) -> (String, String) {
    match serde_json::from_str::<serde_json::Value>(content) {
        Ok(v) => {
            let code = v
                .get("type")
                .and_then(|x| x.as_str())
                .unwrap_or("UNKNOWN")
                .to_string();
            let message = v
                .get("message")
                .and_then(|x| x.as_str())
                .unwrap_or(content)
                .to_string();
            (code, message)
        }
        Err(_) => ("UNKNOWN".to_string(), content.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_error_body_extracts_type_and_message() {
        let content = r#"{"type":"PAYMENT_NOT_FOUND","message":"결제 건을 찾을 수 없습니다"}"#;
        let (code, message) = parse_error_body(content);
        assert_eq!(code, "PAYMENT_NOT_FOUND");
        assert_eq!(message, "결제 건을 찾을 수 없습니다");
    }

    #[test]
    fn parse_error_body_falls_back_on_non_json() {
        let (code, message) = parse_error_body("not json at all");
        assert_eq!(code, "UNKNOWN");
        assert_eq!(message, "not json at all");
    }

    #[test]
    fn parse_error_body_handles_json_without_known_fields() {
        let (code, message) = parse_error_body(r#"{"unexpected": 1}"#);
        assert_eq!(code, "UNKNOWN");
        assert!(message.contains("unexpected"));
    }
}
