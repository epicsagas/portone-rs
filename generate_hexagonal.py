import re
import os
import subprocess
from collections import defaultdict

DEFAULT_API_PATH = "src/adapter/generated_api/default_api.rs"

with open(DEFAULT_API_PATH, "r") as f:
    content = f.read()

# Pattern to match the (possibly multi-line, fmt'd) signature:
#   pub async fn get_payment(
#       configuration: &configuration::Configuration,
#       payment_id: &str,
#       store_id: Option<&str>,
#   ) -> Result<models::Payment, Error<GetPaymentError>> {
# re.DOTALL lets `.*?` span newlines in the args section.
pattern = r"pub async fn (\w+)\(\s*configuration: &configuration::Configuration,?\s*(.*?)\s*\)\s*->\s*Result\s*<\s*(.*?)\s*,\s*Error\s*<\s*(.*?)\s*>\s*,?\s*>"
matches = re.findall(pattern, content, re.DOTALL)


def normalize_args(args):
    """멀티라인 args를 단일 라인 `, ` 구분으로 정규화한다."""
    parts = [a.strip() for a in args.split(",") if a.strip()]
    return ", ".join(parts)


def classify(fn_name):
    """도메인별 분류. platform을 payment보다 먼저 매칭하여
    향후 `get_platform_payments` 같은 함수가 payment로 잘못 분류되는 회귀를 방지한다."""
    if "platform" in fn_name:
        return "platform"
    if "b2b" in fn_name or "tax_invoice" in fn_name:
        return "b2b"
    if "billing_key" in fn_name:
        return "billing_key"
    if "identity_verification" in fn_name:
        return "identity_verification"
    if "payment" in fn_name:
        return "payment"
    return "misc"


groups = defaultdict(list)
for fn_name, args, ret_type, _err_type in matches:
    groups[classify(fn_name)].append((fn_name, args, ret_type))

# Ensure output directories exist
os.makedirs("src/port", exist_ok=True)
os.makedirs("src/adapter", exist_ok=True)

# 알파벳순 도메인 순서로 결정적 생성
DOMAIN_ORDER = sorted(groups.keys())

port_mod = []
adapter_mod = []

for group_name in DOMAIN_ORDER:
    funcs = groups[group_name]
    if not funcs:
        continue
    # 파일명은 도메인 스네이크케이스, trait명은 카멜케이스
    file_stem = group_name
    camel_group = "".join(x.capitalize() for x in group_name.split("_"))
    trait_name = f"{camel_group}Port"

    # 1. Generate Port Trait (returns crate::error::SdkError)
    port_content = [
        "use async_trait::async_trait;",
        "use crate::domain::models;",
        "",
        "#[async_trait]",
        f"pub trait {trait_name} {{",
    ]
    for fn_name, args, ret_type in funcs:
        clean_args = args.replace("models::", "models::")
        clean_ret = ret_type.replace("models::", "models::")
        # 응답 스키마가 빈 object로 정의된(→ serde_json::Value) 엔드포인트에 경고 doc-comment 부착.
        # PortOne OpenAPI 스펙 한계로 typed 모델 생성이 불가하므로 호출자에게 raw JSON임을 알린다.
        if clean_ret == "serde_json::Value":
            port_content.append(
                "    /// ⚠️ 응답 스키마가 미정의(빈 object)되어 raw JSON(`serde_json::Value`)을 반환합니다. PortOne OpenAPI 스펙 한계."
            )
        port_content.append(
            f"    async fn {fn_name}(&self, {clean_args}) -> Result<{clean_ret}, crate::error::SdkError>;"
        )
    port_content.append("}")

    with open(f"src/port/{file_stem}_port.rs", "w") as f:
        f.write("\n".join(port_content))
    port_mod.append(f"pub mod {file_stem}_port;")
    port_mod.append(f"pub use {file_stem}_port::*;")

    # 2. Generate Adapter Impl (maps generated Error<T> to SdkError)
    adapter_content = [
        "use async_trait::async_trait;",
        "use crate::domain::models;",
        "use crate::adapter::generated_api::default_api;",
        "use crate::adapter::http_client::HttpClient;",
        f"use crate::port::{trait_name};",
        "",
        "#[async_trait]",
        f"impl {trait_name} for HttpClient {{",
    ]
    for fn_name, args, ret_type in funcs:
        clean_args = args.replace("models::", "models::")
        clean_ret = ret_type.replace("models::", "models::")

        # extract param names: e.g. "payment_id: &str, body: models::Body" -> "payment_id, body"
        arg_names = []
        if args.strip():
            for arg in args.split(","):
                arg = arg.strip()
                if arg:
                    name = arg.split(":")[0].strip()
                    arg_names.append(name)
        call_args = ", ".join(arg_names)
        if call_args:
            call_args = ", " + call_args

        adapter_content.append(
            f"    async fn {fn_name}(&self, {clean_args}) -> Result<{clean_ret}, crate::error::SdkError> {{"
        )
        adapter_content.append(
            f"        default_api::{fn_name}(&self.configuration{call_args})"
        )
        adapter_content.append("            .await")
        adapter_content.append("            .map_err(crate::error::SdkError::from)")
        adapter_content.append("    }")
    adapter_content.append("}")

    with open(f"src/adapter/{file_stem}_adapter.rs", "w") as f:
        f.write("\n".join(adapter_content))
    adapter_mod.append(f"pub mod {file_stem}_adapter;")

# Write port mod.rs (alphabetical, deterministic)
with open("src/port/mod.rs", "w") as f:
    f.write("\n".join(port_mod))

# Write adapter mod.rs (alphabetical, deterministic)
with open("src/adapter/mod.rs", "w") as f:
    adapter_content = [
        "pub mod generated_api;",
        "pub mod http_client;",
    ] + sorted(adapter_mod) + [
        "pub use http_client::*;",
    ]
    f.write("\n".join(adapter_content))

# === 회귀 게이트 (P2) ===
# 생성된 port/adapter 메서드 수 합이 default_api의 async fn 수와 정확히 일치해야 한다.
# 일치하지 않으면 정규식 매칭이 누락을 일으킨 것(OpenAPI 갱신 시 출력 포맷 변경 등).
generated_fn_count = len(matches)
generated_method_count = sum(len(funcs) for funcs in groups.values())
assert (
    generated_method_count == generated_fn_count
), (
    f"REGRESSION GATE FAILED: classified {generated_method_count} methods but "
    f"found {generated_fn_count} async fns in default_api.rs — "
    "classification regex missed endpoints, check the match pattern."
)

print(
    f"Successfully generated Hexagonal bindings: {generated_fn_count} endpoints "
    f"across {len(groups)} domains ({', '.join(DOMAIN_ORDER)})."
)

# 생성 직후 포맷을 적용하여 커밋된 파일과 항상 일치(멱등)하도록 한다.
subprocess.run(["cargo", "fmt"], check=True)
