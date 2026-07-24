**[한국어](CONTRIBUTING.md)** | [English](CONTRIBUTING.en.md)

# PortOne Rust SDK 기여하기

PortOne Rust SDK 프로젝트에 기여해 주셔서 감사드립니다!

## 개발 환경

- **Rust** (stable, edition 2021, 최소 1.75)
- **Python 3** (포트/어댑터/테스트 자동 생성 스크립트용)
- **openapi-generator-cli** (명세 갱신 시에만 필요 — [openapitools.json](openapitools.json), 버전 7.24.0)

## 기본 워크플로

1. 저장소를 포크합니다.
2. 새 브랜치를 생성합니다. (`git checkout -b feature/amazing-feature`)
3. 코드를 수정하거나 기능을 추가합니다.
4. 아래 검증 명령을 모두 통과하는지 확인합니다.
5. 변경 사항을 커밋합니다. (`git commit -m 'feat: add some amazing feature'`)
6. 브랜치에 푸시하고 풀 리퀘스트를 생성합니다.

## 검증 명령 (모두 통과해야 함)

```bash
cargo fmt                                          # 포맷
cargo fmt -- --check                                # 포맷 확인 (CI 통과 조건)
cargo clippy --all-targets -- -D warnings           # 린트 (경고를 에러로)
cargo test                                          # 단위 + 통합 + 자동 생성 테스트 전체
cargo build --examples                              # examples/ 컴파일 (README 예제 포함)
```

CI([.github/workflows/ci.yml](.github/workflows/ci.yml))는 `build`, `test`, `rustfmt --check`, `clippy -D warnings` 4게이트를 실행합니다.

## 생성 파일에 대하여

다음 파일은 **자동 생성**되므로 직접 수정하지 마세요:

- `src/adapter/generated_api/` — OpenAPI Generator 산출물
- `src/domain/models/` — OpenAPI Generator 산출물
- `src/port/*_port.rs` — `generate_hexagonal.py` 산출물
- `src/adapter/*_adapter.rs` — `generate_hexagonal.py` 산출물
- `src/port/mod.rs`, `src/adapter/mod.rs` — `generate_hexagonal.py` 산출물
- `tests/generated/endpoints.rs` — `generate_tests.py` 산출물
- `tests/generated/endpoints_success.rs` — `generate_e2e_tests.py` 산출물(성공 200 경로; oneOf enum 응답은 round-trip 결함으로 skip)

수동으로 작성/수정해야 하는 파일:
- `src/lib.rs`, `src/error.rs`, `src/adapter/http_client.rs` — 코어 로직
- `generate_hexagonal.py`, `generate_tests.py`, `generate_e2e_tests.py` — 생성기 자체
- `examples/`, `tests/*.rs` (generated 제외), `README*.md`, `CONTRIBUTING*.md` — 예제/문서

## 코드 재생성

OpenAPI 명세([docs/portone-v2-openapi.json](docs/portone-v2-openapi.json))를 갱신한 후:

```bash
# 1. openapi-generator-cli 로 generated_api/ 와 models/ 재생성
openapi-generator-cli generate -g rust -i docs/portone-v2-openapi.json \
  -o target/generated --additional-properties=...

# 2. 포트/어댑터 재생성 (회귀 게이트 내장)
python3 generate_hexagonal.py

# 3. 회귀 테스트 재생성
python3 generate_tests.py

# 4. 전체 검증
cargo fmt && cargo test && cargo clippy --all-targets -- -D warnings
```

`generate_hexagonal.py` 와 `generate_tests.py` 는 생성 직후 자기 검증(assert)을 수행합니다. OpenAPI 갱신으로 함수 분류나 경로 추출이 누락되면 스크립트가 즉시 실패합니다.

## 커밋 메시지 규칙

[Conventional Commits](https://www.conventionalcommits.org/) 형식을 따릅니다:

```
type(scope): description
```

예: `feat(payment): add billing key payment support`, `fix(b2b): correct tax invoice path`.

## 번역 기여하기

번역 관련 기여 시 [번역 PR 템플릿](.github/PULL_REQUEST_TEMPLATE/translation.md)을 참조하세요:
- 최신 한국어 버전을 기준으로 번역합니다.
- 적절한 경우 기술 용어는 원어(또는 영어)로 유지합니다.
- 코드 블록은 변경하지 않습니다.

## 라이선스

이 프로젝트에 기여함으로써 귀하의 기여 내용이 프로젝트의 라이선스(MIT 또는 Apache-2.0)를 따르는 것에 동의합니다.
