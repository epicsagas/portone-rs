[한국어](CONTRIBUTING.md) | **[English](CONTRIBUTING.en.md)**

> This is a translation of [CONTRIBUTING.md](CONTRIBUTING.md).
> The Korean version is the authoritative source and may be more up-to-date.

# Contributing to PortOne Rust SDK

Thank you for contributing to the PortOne Rust SDK!

## Development environment

- **Rust** (stable, edition 2021, minimum 1.75)
- **Python 3** (for the port/adapter/test generation scripts)
- **openapi-generator-cli** (only needed when updating the spec — [openapitools.json](openapitools.json), version 7.24.0)

## Basic workflow

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/amazing-feature`).
3. Make your changes.
4. Make sure all verification commands below pass.
5. Commit your changes (`git commit -m 'feat: add some amazing feature'`).
6. Push and open a pull request.

## Verification commands (all must pass)

```bash
cargo fmt                                          # format
cargo fmt -- --check                                # verify formatting (CI gate)
cargo clippy --all-targets -- -D warnings           # lint (warnings as errors)
cargo test                                          # unit + integration + generated tests
cargo build --examples                              # compile examples/ (covers README samples)
```

CI ([.github/workflows/ci.yml](.github/workflows/ci.yml)) runs four gates: `build`, `test`, `rustfmt --check`, and `clippy -D warnings`.

## About generated files

The following files are **auto-generated** — do not edit them by hand:

- `src/adapter/generated_api/` — OpenAPI Generator output
- `src/domain/models/` — OpenAPI Generator output
- `src/port/*_port.rs` — output of `generate_hexagonal.py`
- `src/adapter/*_adapter.rs` — output of `generate_hexagonal.py`
- `src/port/mod.rs`, `src/adapter/mod.rs` — output of `generate_hexagonal.py`
- `tests/generated/endpoints.rs` — output of `generate_tests.py`
- `tests/generated/endpoints_success.rs` — output of `generate_e2e_tests.py` (success 200 path; oneOf-enum responses skipped due to a round-trip defect)

Files you write or edit by hand:
- `src/lib.rs`, `src/error.rs`, `src/adapter/http_client.rs` — core logic
- `generate_hexagonal.py`, `generate_tests.py`, `generate_e2e_tests.py` — the generators themselves
- `examples/`, `tests/*.rs` (except generated), `README*.md`, `CONTRIBUTING*.md` — examples/docs

## Regenerating code

After updating the OpenAPI spec ([docs/portone-v2-openapi.json](docs/portone-v2-openapi.json)):

```bash
# 1. Regenerate generated_api/ and models/ with openapi-generator-cli
openapi-generator-cli generate -g rust -i docs/portone-v2-openapi.json \
  -o target/generated --additional-properties=...

# 2. Regenerate ports/adapters (built-in regression gate)
python3 generate_hexagonal.py

# 3. Regenerate regression tests
python3 generate_tests.py

# 4. Full verification
cargo fmt && cargo test && cargo clippy --all-targets -- -D warnings
```

Both `generate_hexagonal.py` and `generate_tests.py` self-validate (assert) right after generation. If a spec update causes a classification gap or path-extraction miss, the script fails immediately.

## Commit message convention

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): description
```

Example: `feat(payment): add billing key payment support`, `fix(b2b): correct tax invoice path`.

## Translation contributions

For translation-related contributions, see the [translation PR template](.github/PULL_REQUEST_TEMPLATE/translation.md):
- Translate from the latest Korean version.
- Keep technical terms in the original language (or English) where appropriate.
- Do not modify code blocks.

## License

By contributing, you agree that your contributions are licensed under the project's license (MIT or Apache-2.0).
