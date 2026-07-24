**[한국어](benchmarks.md)** | [English](benchmarks.en.md)

# 벤치마크 및 품질 메트릭

> 최종 갱신: 2026-07-24 · 품질 정비(에러 Io variant·prelude·Value doc-comment·mock 예제)

이 문서는 portone-rs SDK의 **로컬 CPU 벤치마크**, **테스트 커버리지**, **정적 품질 메트릭**을 기록합니다. HTTP API 클라이언트이므로 실제 엔드투엔드 latency는 네트워크/서버에 의존하지만, SDK 자체가 요청·응답마다 수행하는 직렬화·역직렬화·에러 변환 오버헤드는 로컬로 측정 가능하고 회귀 추적 가치가 있습니다.

## 측정 환경

| 항목 | 값 |
|------|-----|
| OS | macOS (Darwin 25.5.0, arm64) |
| Rust | 1.95.0 (stable) |
| 벤치 프레임워크 | criterion 0.5 |
| 실행 모드 | `cargo bench --bench eval_harness -- --quick` |

## 로컬 마이크로벤치

[`benches/eval_harness.rs`](../benches/eval_harness.rs)가 SDK의 핫패스 3종을 측정합니다. 구간은 criterion이 보고하는 중앙값이며, 괄호 안은 95% 신뢰구간 하한/상한입니다.

| 벤치 | 측정 항목 | 중앙값 | 구간 |
|------|----------|-------:|------|
| `sdk_error/from_response_error` | `Error<T>` → `SdkError` 변환 (에러 본문 파싱 포함) | **302 ns** | 294–310 ns |
| `serde/serialize_request_body` | 요청 본문 직렬화 (`CapturePaymentBody`) | **40 ns** | 40–40 ns |
| `serde/deserialize_response_50_items` | 응답 역직렬화 (50개 항목 리스트) | **6.20 µs** | 6.06–6.33 µs |

### 해석

- **에러 변환 (~300 ns)**: 모든 API 에러 응답에서 실행되는 경로. `parse_error_body`의 JSON 파싱이 주를 이룹니다. 사용자 체감에는 무시 가능한 수준입니다.
- **직렬화 (~40 ns)**: 단일 요청 본문 직렬화. 본문이 커질수록 선형 증가하지만, 일반적인 결제 요청에서는 마이크로초 미만입니다.
- **역직렬화 (~6.2 µs / 50개)**: 항목당 약 124 ns. 리스트 응답(결제 내역, 거래처 등)에서 응답 크기에 비례합니다.

> **정비 후 재측정 노트 (2026-07-24):** 위 값은 에러 Io variant·prelude·Value doc-comment·mock 예제 정비 *후* 측정값입니다. 정비 전(276 ns / 34 ns / 5.66 µs) 대비 수치가 약간 높게 나왔으나 모두 p > 0.05(통계적으로 유의하지 않음)이며, 정비와 무관한 serialize/deserialize 벤치도 동반 상승한 것으로 보아 **머신 상태에 의한 측정 노이즈**로 판단합니다. 정비(에러 variant 추가·cfg·doc-comment)는 핫패스 성능에 영향을 주지 않습니다.

## 테스트 커버리지

| 지표 | 값 |
|------|-----|
| 총 테스트 | **264개 통과** (단위/통합 24 + 라우팅 147 + e2e 성공 93) |
| 엔드포인트 회귀 테스트 | **147/147** (라우팅 + 4xx 에러 처리) |
| e2e 성공(200) 테스트 | **93/147** (54개 oneOf enum 응답은 round-trip 결함으로 skip) |

> **eval 보고서 (epic:eval):** [`benchmarks/baselines/latest.json`](../benchmarks/baselines/latest.json) — correctness·performance·quality·regression 전부 **PASS (1.0)**. correctness는 264개 테스트(라우팅 147 + e2e 성공 93 + 단위/통합 24) 기준.
| 자동 생성 테스트 | [`tests/generated/endpoints.rs`](../tests/generated/endpoints.rs) |

자동 생성 테스트는 OpenAPI 갱신으로 엔드포인트 경로/메서드가 바뀌면 즉시 실패하여 회귀를 잡습니다. 생성기는 [`generate_tests.py`](../generate_tests.py)입니다.

## 정적 품질 게이트

모두 green이어야 CI를 통과합니다.

| 게이트 | 명령 | 상태 |
|--------|------|:----:|
| 포맷 | `cargo fmt -- --check` | ✅ |
| 린트 | `cargo clippy --all-targets -- -D warnings` | ✅ (경고 0) |
| 빌드 | `cargo build` + `cargo build --examples` | ✅ |
| 테스트 | `cargo test` | ✅ (170/170) |
| codegen 동기화 | `generate_hexagonal.py` + `generate_tests.py` 후 `git diff --exit-code` | ✅ |

## 코드 규모

| 항목 | 수 |
|------|---:|
| 생성 API 엔드포인트 (`default_api.rs`) | 147 |
| 포트 trait 메서드 (6개 도메인) | 147 |
| 도메인 모델 (`domain/models/`) | 971 |
| 수작성 코드 (lib/error/port/adapter 코어) | ~2,600줄 |
| 자동 생성 코드 (`generated_api/`) | ~10,400줄 |

## 재현

```bash
# 마이크로벤치
cargo bench --bench eval_harness

# 전체 품질 게이트
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

벤치 상세 결과(criterion JSON 추정치)는 `target/criterion/`에 생성됩니다. 본 프로젝트는 criterion을 `default-features = false`로 사용해 HTML/SVG 리포트는 생성되지 않으며, `target/criterion/<벤치>/new/estimates.json`의 median을 읽습니다.
