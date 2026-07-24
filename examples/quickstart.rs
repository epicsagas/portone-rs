//! README 퀵스타트 예제의 컴파일 보장용입니다.
//! `cargo build --examples` 가 통과하면 README 코드 샘플이 항상 빌드됨을 보장합니다.
//!
//! 실제 API 호출은 수행하지 않습니다(자격 증명 없음). 예제의 *형태*가 유효한지만 검증합니다.

use portone_rs::v2::{HttpClient, PaymentPort};

#[tokio::main]
async fn main() -> Result<(), portone_rs::v2::SdkError> {
    // 1. API 시크릿으로 클라이언트를 초기화합니다.
    //    `PortOne <SECRET>` Authorization 헤더가 자동으로 구성됩니다.
    let client = HttpClient::new("YOUR_PORTONE_API_SECRET")?;

    // 2. 엔드포인트를 호출합니다. HttpClient는 모든 포트 트레이트를 구현합니다.
    let payment_id = "test-payment-id-1234";
    let _response = client.get_payment(payment_id, None).await?;

    // 3. 응답을 처리합니다.
    //    println!("{_response:?}");

    Ok(())
}
