//! 로컬 마이크로벤치 — portone-rs SDK의 CPU 바운드 핫패스를 측정합니다.
//!
//! HTTP API 클라이언트이므로 실제 엔드투엔드 latency는 네트워크/서버에 의존하지만,
//! SDK 자체가 요청/응답마다 수행하는 직렬화·역직렬화·에러 변환 오버헤드는
//! 로컬로 측정 가능하고 회귀 추적 가치가 있습니다.
//!
//! 실행: `cargo bench --bench eval_harness`

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use portone_rs::v2::adapter::generated_api::{Error, ResponseContent};
use portone_rs::v2::domain::models::{
    Bank, BankInfo, BankInfoName, CapturePaymentBody, GetBankInfosResponse,
};
use portone_rs::v2::SdkError;

/// 모든 API 에러 응답에서 실행되는 Error<T> → SdkError 변환 (parse_error_body 포함).
fn bench_error_conversion(c: &mut Criterion) {
    let body = r#"{"type":"PAYMENT_NOT_FOUND","message":"결제 건을 찾을 수 없습니다"}"#;
    c.bench_function("sdk_error/from_response_error", |b| {
        b.iter(|| {
            let rc = ResponseContent {
                status: reqwest::StatusCode::NOT_FOUND,
                content: black_box(body).to_string(),
                entity: None::<()>,
            };
            let _converted: SdkError = Error::<()>::ResponseError(rc).into();
        })
    });
}

/// 요청 본문 직렬화 — 모든 POST/PATCH/PUT 요청에서 실행.
fn bench_serialize_request(c: &mut Criterion) {
    let body = CapturePaymentBody {
        store_id: Some("store-42".to_string()),
    };
    c.bench_function("serde/serialize_request_body", |b| {
        b.iter(|| {
            let _json: String = serde_json::to_string(black_box(&body)).unwrap();
        })
    });
}

/// 응답 역직렬화 — 50개 항목 리스트(대표 규모) 파싱.
fn bench_deserialize_response(c: &mut Criterion) {
    let items: Vec<BankInfo> = (0..50)
        .map(|_| BankInfo {
            bank: Bank::Kookmin,
            name: Box::new(BankInfoName {
                ko: "국민은행".to_string(),
            }),
        })
        .collect();
    let resp = GetBankInfosResponse { items };
    let json = serde_json::to_string(&resp).unwrap();

    c.bench_function("serde/deserialize_response_50_items", |b| {
        b.iter(|| {
            let _parsed: GetBankInfosResponse = serde_json::from_str(black_box(&json)).unwrap();
        })
    });
}

criterion_group!(
    benches,
    bench_error_conversion,
    bench_serialize_request,
    bench_deserialize_response,
);
criterion_main!(benches);
