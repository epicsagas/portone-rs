# PortOne Rust SDK (V2) (Unofficial)

> ⚠️ **Disclaimer**
> This library is an **unofficial** Rust SDK developed independently by an individual. It is not officially supported or maintained by PortOne Inc. The trademarks "PortOne" and "포트원" belong to Korea Port One Co., Ltd. For official documentation and services, please visit [PortOne Developer Center](https://developers.portone.io/).

> 🧪 **Production Use Caution**
> This SDK has not undergone integration testing against the live PortOne API. All regression and e2e tests run on wiremock-based mocks, so please validate thoroughly against the live environment before production adoption with real payment funds. **The developer assumes no liability for any losses arising from the use of this library.**

> 🚫 **Liability**
> The original author and the company in question (PortOne) assume no responsibility for any financial or material losses incurred while using this library.

---

[한국어](README.md) | **[English](README.en.md)**

> Original API reference: https://developers.portone.io/api/rest-v2?v=v2

An unofficial Rust SDK for the PortOne V2 API built with **Hexagonal Architecture (Ports & Adapters Pattern)**.

It provides a single, consistent API for **147 endpoints** and **971 data models** generated from the PortOne API (v1.16.0) OpenAPI 3.0.3 spec.

## Key Features

- **Single Error Type**: Every call returns `SdkError` — unifying initialization and API call errors.
- **Auto Authentication**: Automatically constructs the `PortOne <SECRET>` Authorization header.
- **Domain-Specific Ports**: Separated traits for Payment, Platform, B2B, Billing Key, and Identity Verification.
- **Exhaustive Regression Tests**: 147 endpoints verified via auto-generated test suite.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
portone-rs = "0.1.0"
```

## Quick start

Initialize an `HttpClient` with your PortOne V2 API secret, then call any port trait method.

```rust
use portone_rs::v2::{HttpClient, PaymentPort};

#[tokio::main]
async fn main() -> Result<(), portone_rs::v2::SdkError> {
    // 1. Initialize the client with your API secret.
    //    The `PortOne <SECRET>` Authorization header is configured automatically.
    let client = HttpClient::new("YOUR_PORTONE_API_SECRET")?;

    // 2. Call an endpoint. HttpClient implements all port traits.
    let payment_id = "test-payment-id-1234";
    let response = client.get_payment(payment_id, None).await?;

    // 3. Process the response.
    println!("Payment details: {response:?}");

    Ok(())
}
```

> The example above mirrors [`examples/quickstart.rs`](examples/quickstart.rs), which is compiled by CI via `cargo build --examples` to guarantee the README sample always builds.

> 💡 Bring commonly used types in at once with `use portone_rs::prelude::*;`

## Authentication

`HttpClient::new(api_secret)` turns the secret into a `PortOne <SECRET>` Authorization header attached as a default header to every request. The header value is marked sensitive so it is masked in logs.

```rust
let client = HttpClient::new("YOUR_PORTONE_API_SECRET")?
    .with_base_url("https://api.portone.io"); // default; override for testing/private endpoints
```

Use `with_base_url` to point at a mock server in tests or a private endpoint.

## Domain ports

`HttpClient` implements six port traits, each importable directly from `portone_rs::v2::`.

| Trait | Category | Methods |
|-------|----------|---------|
| [`PaymentPort`](src/port/payment_port.rs) | Payment search/approve/cancel, payment sessions, cash receipts | 23 |
| [`PlatformPort`](src/port/platform_port.rs) | Platform partners/contracts/settlements/transfers | 54 |
| [`B2bPort`](src/port/b2b_port.rs) | Tax invoice issuance/search, counterparty management | 31 |
| [`BillingKeyPort`](src/port/billing_key_port.rs) | Billing key issue/search/delete, billing key payments | 7 |
| [`IdentityVerificationPort`](src/port/identity_verification_port.rs) | Identity verification request/search/resend | 5 |
| [`MiscPort`](src/port/misc_port.rs) | Escrow, bank info, promotions, auth/token refresh | 27 |

```rust
use portone_rs::v2::{HttpClient, B2bPort, BillingKeyPort, MiscPort};
// Import only the traits you need.
```

## Error Handling

All errors are unified under a single `SdkError` type.

```rust
use portone_rs::v2::SdkError;

match client.get_payment("missing-id", None).await {
    Ok(payment) => println!("{payment:?}"),
    Err(SdkError::ApiError { status, code, message }) => {
        // An error returned by the PortOne API (status, code, message)
        println!("API error {status}: {code} - {message}");
    }
    Err(SdkError::HttpError(e)) => eprintln!("Transport error: {e}"),
    Err(e) => eprintln!("Other error: {e}"),
}
```

Match a specific error case via `ApiError.code`:

```rust
if let Err(SdkError::ApiError { code, .. }) = client.get_payment("id", None).await {
    if code == "PAYMENT_NOT_FOUND" {
        // handle the "payment does not exist" case
    }
}
```

| Variant | Meaning |
|---------|---------|
| `HttpError` | reqwest transport-layer error (timeout, connection failure, …) |
| `InvalidHeaderValue` | Secret contained characters not allowed in a header |
| `SerializationError` | Request/response serialization failure |
| `Io` | I/O error from the transport layer |
| `ApiError { status, code, message }` | Non-2xx response returned by the PortOne API |
| `Unknown` | Unclassified error |

## Architecture (Hexagonal / Ports & Adapters)

```
src/
├── domain/models/      # 971 data structures generated from the OpenAPI spec
├── port/               # Per-domain traits (the abstract boundary)
│   ├── payment_port.rs
│   ├── platform_port.rs
│   └── ...
├── adapter/
│   ├── http_client.rs  # Concrete implementation of the port traits (entry point)
│   ├── *_adapter.rs    # Wraps the generated default_api calls into SdkError
│   └── generated_api/  # OpenAPI Generator output (reqwest-based)
└── error.rs            # Single SdkError definition
```

- **`port/`** defines *what* can be done (domain abstraction).
- **`adapter/`** implements *how* it is done over HTTP (transport detail).
- **`generated_api/`** is generated code that you never edit by hand.

> **Note:** 14 endpoints return `serde_json::Value` because the PortOne OpenAPI spec defines their response as an empty object, making a typed model impossible. Those trait methods are marked with a `⚠️` doc-comment.

Thanks to this separation, tests only need to point an `HttpClient` at a mock server.

## Code generation

The SDK is produced in stages:

1. **OpenAPI Generator** (`openapitools.json`, version 7.24.0) emits the reqwest client under `src/adapter/generated_api/` and models under `src/domain/models/`.
2. **`generate_hexagonal.py`** parses the generated client to build the `port/` traits and `adapter/` impls, with a regression gate that catches any classification gap.
3. **`generate_tests.py`** generates routing regression tests for all 147 endpoints into `tests/generated/endpoints.rs`.

After updating the OpenAPI spec:

```bash
# (regenerate generated_api/ and models/ with openapi-generator-cli, then:)
python3 generate_hexagonal.py   # regenerate port/adapter
python3 generate_tests.py        # regenerate regression tests
cargo test                       # full verification
```

## Performance & quality

Local microbenchmarks (criterion) and quality metrics are summarized in [docs/benchmarks.en.md](docs/benchmarks.en.md).

- **Microbenchmarks** (median): error conversion ~300 ns · request serialization ~40 ns · response deserialization ~6.2 µs (50 items)
- **Tests**: 264 passing (routing 147 + e2e success 93 + unit/integration 24)
- **Eval report**: epic:eval — all 4 dimensions (correctness·performance·quality·regression) PASS — [`benchmarks/baselines/latest.json`](benchmarks/baselines/latest.json)
- **Static gates**: `cargo fmt`, `cargo clippy -D warnings`, `cargo test` all green

## Contributing

See [CONTRIBUTING.en.md](CONTRIBUTING.en.md) for contribution and testing/format rules. Quick summary:

```bash
cargo fmt                # format
cargo clippy --all-targets -- -D warnings   # lint
cargo test               # existing + generated tests, all
```

## License

[Apache-2.0](LICENSE)
