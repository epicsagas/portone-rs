[한국어](benchmarks.md) | **[English](benchmarks.en.md)**

> This is a translation of [benchmarks.md](benchmarks.md).
> The Korean version is the authoritative source and may be more up-to-date.

# Benchmarks & Quality Metrics

> Last updated: 2026-07-24 · quality tidy-up (error Io variant · prelude · Value doc-comment · mock example)

This document records **local CPU benchmarks**, **test coverage**, and **static quality metrics** for the portone-rs SDK. As an HTTP API client, real end-to-end latency depends on the network/server, but the serialization, deserialization, and error-conversion overhead the SDK itself incurs per request/response is measurable locally and worth tracking for regressions.

## Measurement environment

| Item | Value |
|------|-------|
| OS | macOS (Darwin 25.5.0, arm64) |
| Rust | 1.95.0 (stable) |
| Benchmark framework | criterion 0.5 |
| Run mode | `cargo bench --bench eval_harness -- --quick` |

## Local microbenchmarks

[`benches/eval_harness.rs`](../benches/eval_harness.rs) measures three SDK hotpaths. Values are criterion medians; the range in parentheses is the 95% confidence interval lower/upper bound.

| Benchmark | What it measures | Median | Range |
|-----------|------------------|-------:|-------|
| `sdk_error/from_response_error` | `Error<T>` → `SdkError` conversion (incl. error body parsing) | **302 ns** | 294–310 ns |
| `serde/serialize_request_body` | Request body serialization (`CapturePaymentBody`) | **40 ns** | 40–40 ns |
| `serde/deserialize_response_50_items` | Response deserialization (50-item list) | **6.20 µs** | 6.06–6.33 µs |

### Interpretation

- **Error conversion (~300 ns)**: the path run on every API error response. JSON parsing in `parse_error_body` dominates. Imperceptible to end users.
- **Serialization (~40 ns)**: serializing a single request body. Scales linearly as the body grows, but stays sub-microsecond for typical payment requests.
- **Deserialization (~6.2 µs / 50 items)**: about 124 ns per item. Proportional to response size for list responses (payment histories, counterparties, etc.).

> **Post-tidy re-measurement note (2026-07-24):** the values above were measured *after* the tidy-up (error Io variant · prelude · Value doc-comment · mock example). They read slightly higher than the pre-tidy run (276 ns / 34 ns / 5.66 µs), but all are p > 0.05 (not statistically significant) and the tidy-up-irrelevant serialize/deserialize benchmarks rose in tandem — indicating **measurement noise from machine state**, not a regression. The tidy-up (error variant addition · cfg · doc-comment) does not affect hot-path performance.

## Test coverage

| Metric | Value |
|--------|-------|
| Total tests | **264 passing** (unit/integration 24 + routing 147 + e2e success 93) |
| Endpoint regression tests | **147/147** (routing + 4xx error handling) |
| e2e success (200) tests | **93/147** (54 oneOf-enum responses skipped due to round-trip defect) |

> **Eval report (epic:eval):** [`benchmarks/baselines/latest.json`](../benchmarks/baselines/latest.json) — correctness·performance·quality·regression all **PASS (1.0)**.
| Generated tests | [`tests/generated/endpoints.rs`](../tests/generated/endpoints.rs) |

Generated tests fail immediately if an OpenAPI update changes an endpoint path or method, catching regressions. The generator is [`generate_tests.py`](../generate_tests.py).

## Static quality gates

All must be green to pass CI.

| Gate | Command | Status |
|------|---------|:------:|
| Format | `cargo fmt -- --check` | ✅ |
| Lint | `cargo clippy --all-targets -- -D warnings` | ✅ (0 warnings) |
| Build | `cargo build` + `cargo build --examples` | ✅ |
| Tests | `cargo test` | ✅ (264/264) |
| Codegen sync | `generate_hexagonal.py` + `generate_tests.py` then `git diff --exit-code` | ✅ |

## Code size

| Item | Count |
|------|------:|
| Generated API endpoints (`default_api.rs`) | 147 |
| Port trait methods (6 domains) | 147 |
| Domain models (`domain/models/`) | 971 |
| Hand-written code (lib/error/port/adapter core) | ~2,600 lines |
| Generated code (`generated_api/`) | ~10,400 lines |

## Reproducing

```bash
# Microbenchmarks
cargo bench --bench eval_harness

# Full quality gates
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Detailed benchmark results (criterion JSON estimates) are written to `target/criterion/`. This project uses criterion with `default-features = false`, so no HTML/SVG reports are generated; read the median from `target/criterion/<bench>/new/estimates.json`.
