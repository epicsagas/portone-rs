# PortOne Rust SDK (V2) (Unofficial)

> ⚠️ **중요 공지 (Disclaimer)**
> 본 라이브러리는 개인이 개발한 **비공식(Unofficial)** Rust SDK이며, 주식회사 코리아포트원(PortOne)의 공식 지원이나 관리를 받지 않는 독립적인 오픈소스 프로젝트입니다.
> 'PortOne' 및 '포트원' 관련 상표명에 대한 권리는 주식회사 코리아포트원에 있습니다.
> 공식 정보 및 문서는 [포트원 개발자센터](https://developers.portone.io/)를 참고하시기 바랍니다.

> 🧪 **프로덕션 사용 주의**
> 이 SDK는 실제 PortOne 운영 환경(라이브 API) 통합 테스트를 거치지 않았습니다. 모든 회귀·e2e 테스트는 wiremock 기반 모킹으로 수행되었으므로, 실제 결제 자금이 오가는 프로덕션 도입 전 반드시 라이브 환경에서 사전 검증을 거치시기 바랍니다. **본 라이브러리 사용으로 인해 발생하는 손실에 대해 개발자는 책임을 지지 않습니다.**

> 🚫 **면책 조항 (Liability)**
> 본 라이브러리를 사용하다가 발생하는 금전적·물질적 손실에 대해 원작자 및 해당 기업(포트원)은 일절 책임지지 않습니다.

---

**[한국어](README.md)** | [English](README.en.md)

> 원본 API 명세: https://developers.portone.io/api/rest-v2?v=v2

PortOne V2 API를 위한 비공식 Rust SDK입니다. **헥사고날 아키텍처(포트와 어댑터 패턴)** 로 설계되어 도메인 로직과 HTTP 전송 계층이 깔끔하게 분리되어 있습니다.

PortOne API(버전 1.16.0)의 OpenAPI 3.0.3 명세에서 자동 생성된 **147개 엔드포인트**와 **971개 데이터 모델**을 단일하고 일관된 API로 제공합니다.

## 주요 기능

- **단일 에러 타입**: 모든 호출이 `SdkError` 하나만 반환 — 초기화와 API 호출 에러를 통합.
- **자동 인증**: `PortOne <SECRET>` Authorization 헤더를 자동으로 구성.
- **도메인별 포트**: 결제·플랫폼·B2B·빌링키·본인인증·기능을 각각의 trait으로 분리.
- **전수 회귀 테스트**: 147개 엔드포인트의 라우팅/에러 처리를 자동 생성 테스트로 검증.

## 설치

`Cargo.toml`에 추가하세요:

```toml
[dependencies]
portone-rs = "0.1.0"
```

## 빠른 시작

발급받은 PortOne V2 API 시크릿으로 `HttpClient`를 초기화하면 모든 포트 trait 메서드를 사용할 수 있습니다.

```rust
use portone_rs::v2::{HttpClient, PaymentPort};

#[tokio::main]
async fn main() -> Result<(), portone_rs::v2::SdkError> {
    // 1. API 시크릿으로 클라이언트를 초기화합니다.
    //    `PortOne <SECRET>` Authorization 헤더가 자동으로 구성됩니다.
    let client = HttpClient::new("YOUR_PORTONE_API_SECRET")?;

    // 2. 엔드포인트를 호출합니다. HttpClient는 모든 포트 트레이트를 구현합니다.
    let payment_id = "test-payment-id-1234";
    let response = client.get_payment(payment_id, None).await?;

    // 3. 응답을 처리합니다.
    println!("결제 상세 정보: {response:?}");

    Ok(())
}
```

> 위 예제는 [`examples/quickstart.rs`](examples/quickstart.rs)와 동일하며, CI에서 `cargo build --examples` 로 항상 컴파일됨을 보장합니다.

> 💡 자주 사용하는 타입은 프렐루드로 한 번에 가져올 수 있습니다: `use portone_rs::prelude::*;`

## 인증

`HttpClient::new(api_secret)` 는 시크릿을 `PortOne <SECRET>` 형태의 Authorization 헤더로 변환하여 모든 요청에 기본 헤더로 포함시킵니다. 헤더 값은 민감 정보로 표시되어 로그에 마스킹됩니다.

```rust
let client = HttpClient::new("YOUR_PORTONE_API_SECRET")?
    .with_base_url("https://api.portone.io"); // 기본값, 필요 시 오버라이드
```

`with_base_url` 은 테스트(mock 서버)나 사설 엔드포인트에 연결할 때 사용합니다.

## 도메인 포트

`HttpClient`는 6개의 포트 trait을 구현합니다. 각 trait은 `portone_rs::v2::` 최상위에서 직접 임포트할 수 있습니다.

| 포트 Trait | 주요 기능 |
| --- | --- |
| `PaymentPort` | 결제 조회, 결제 취소, 수동 승인, 결제 예약/취소, 가상계좌 발급 등 |
| `BillingKeyPort` | 빌링키 발급, 빌링키로 결제, 빌링키 단건 조회, 삭제 |
| `IdentityVerificationPort` | 본인인증 요청, 단건 조회, 재전송 |
| `B2bPort` | B2B 세금계산서 작성, 발행, 팩스 전송 등 |
| `PlatformPort` | 파트너, 정산, 플랫폼 계약/추가수수료/할인분담 정책 관리 |
| `MiscPort` | 은행 목록 조회, PG 카드 프로모션 조회, 웹훅 재전송 |

```rust
use portone_rs::v2::{HttpClient, B2bPort, BillingKeyPort, MiscPort};

let client = HttpClient::new("YOUR_SECRET")?;

// 은행 목록 조회
let banks = client.get_bank_infos().await?;

// 빌링키로 결제
```

## 에러 처리

모든 오류는 단일 `SdkError`로 표현됩니다.

```rust
use portone_rs::v2::SdkError;

match client.get_payment("missing-id", None).await {
    Ok(payment) => println!("{payment:?}"),
    Err(SdkError::ApiError { status, code, message }) => {
        // PortOne API가 반환한 에러 (status, code, message)
        println!("API 에러 {status}: {code} - {message}");
    }
    Err(SdkError::HttpError(e)) => eprintln!("전송 계층 오류: {e}"),
    Err(e) => eprintln!("기타 오류: {e}"),
}
```

`ApiError.code` 로 특정 에러 케이스를 매칭할 수 있습니다.

```rust
if let Err(SdkError::ApiError { code, .. }) = client.get_payment("id", None).await {
    if code == "PAYMENT_NOT_FOUND" {
        // 결제 건이 존재하지 않는 경우 처리
    }
}
```

| Variant | 의미 |
|---------|------|
| `HttpError` | reqwest 전송 계층 오류(타임아웃, 연결 실패 등) |
| `InvalidHeaderValue` | 시크릿에 헤더에 사용할 수 없는 문자가 포함된 경우 |
| `SerializationError` | 요청/응답 직렬화 실패 |
| `Io` | 전송 계층의 입출력 오류 |
| `ApiError { status, code, message }` | PortOne API가 반환한 비-2xx 응답 |
| `Unknown` | 분류되지 않은 오류 |

## 아키텍처 (Hexagonal / Ports & Adapters)

```
src/
├── domain/models/      # OpenAPI 명세에서 생성된 971개 데이터 구조
├── port/               # 도메인별 trait (추상 경계)
│   ├── payment_port.rs
│   ├── platform_port.rs
│   └── ...
├── adapter/
│   ├── http_client.rs  # 포트 trait의 구체적 구현 (입구)
│   ├── *_adapter.rs    # 생성된 default_api 호출을 SdkError로 래핑
│   └── generated_api/  # OpenAPI Generator 산출물 (reqwest 기반)
└── error.rs            # 단일 SdkError 정의
```

- **`port/`** 는 "무엇을" 할 수 있는지를 정의(도메인 추상).
- **`adapter/`** 는 "어떻게" HTTP로 수행하는지를 구현(전송 세부사항).
- **`generated_api/`** 는 OpenAPI에서 자동 생성된 코드로, 직접 수정하지 않습니다.

> **참고:** 14개 엔드포인트는 PortOne OpenAPI 스펙의 응답이 빈 object로 정의되어 있어 typed 모델 생성이 불가해 `serde_json::Value`를 반환합니다. 해당 trait 메서드에는 `⚠️` doc-comment로 표시됩니다.

이 분리 덕분에 테스트에서는 mock 서버를 향해 `HttpClient`를 구성하기만 하면 됩니다.

## 코드 생성

이 SDK는 두 단계로 생성됩니다:

1. **OpenAPI Generator** (`openapitools.json`, 버전 7.24.0)가 `src/adapter/generated_api/` 의 reqwest 클라이언트와 `src/domain/models/` 의 모델을 생성합니다.
2. **`generate_hexagonal.py`** 가 생성된 클라이언트를 파싱해 `port/` trait과 `adapter/` 구현을 자동 구성합니다. 스크립트는 회귀 게이트를 포함해 분류 누락을 즉시 감지합니다.
3. **`generate_tests.py`** 가 147개 엔드포인트의 라우팅 회귀 테스트를 `tests/generated/endpoints.rs` 에 생성합니다.

OpenAPI 명세를 갱신한 후:

```bash
# (openapi-generator-cli 로 generated_api/ 와 models/ 재생성 후)
python3 generate_hexagonal.py   # port/adapter 재생성
python3 generate_tests.py        # 회귀 테스트 재생성
cargo test                       # 전체 검증
```

## 성능 및 품질

로컬 마이크로벤치(criterion)와 품질 메트릭은 [docs/benchmarks.md](docs/benchmarks.md)에 정리되어 있습니다.

- **마이크로벤치** (중앙값): 에러 변환 ~300 ns · 요청 직렬화 ~40 ns · 응답 역직렬화 ~6.2 µs(50개 항목)
- **테스트**: 264개 통과 (라우팅 147 + e2e 성공 93 + 단위/통합 24)
- **eval 보고서**: epic:eval 4개 dimension(correctness·performance·quality·regression) 전부 PASS — [`benchmarks/baselines/latest.json`](benchmarks/baselines/latest.json)
- **정적 게이트**: `cargo fmt`, `cargo clippy -D warnings`, `cargo test` 모두 green

## 기여하기

기여 방법과 테스트/포맷 규칙은 [CONTRIBUTING.md](CONTRIBUTING.md)를 참고하세요. 핵심 요약:

```bash
cargo fmt                # 포맷
cargo clippy --all-targets -- -D warnings   # 린트
cargo test               # 기존 + 자동 생성 테스트 전체
```

## 라이선스

[Apache-2.0](LICENSE)
