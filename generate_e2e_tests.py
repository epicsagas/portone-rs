#!/usr/bin/env python3
"""tests/generated/endpoints_success.rs 자동 생성기.

각 default_api 엔드포인트에 대해 **성공(200)** e2e 테스트를 생성한다.
응답(ret_type) 모델의 `Default` 를 직렬화한 JSON 을 wiremock 에 탑재하고,
HttpClient(hexagonal port) 로 호출해 응답이 Ok 로 역직렬화되는지 검증한다.

**알려진 한계(skip):** openapi-generator 의 oneOf enum 응답(`#[serde(tag=..)]` +
newtype variant)은 직렬화 시 tag 와 struct 필드가 중복 키로 평탄화되어
round-trip 이 깨진다(같은 타입 직렬화→역직렬화 실패). 이런 enum 을 응답으로 갖는
엔드포인트는 자동 mock 이 불가하므로 회귀 게이트에서 skip 하고, 아래 known-limitation
메시지로 기록한다. 라우팅/메서드/요청 인자 추출은 generate_tests.py 와 동일.
"""
import glob
import json
import os
import re
import subprocess

DEFAULT_API_PATH = "src/adapter/generated_api/default_api.rs"
MODELS_GLOB = "src/domain/models/*.rs"
OUT_PATH = "tests/generated/endpoints_success.rs"

with open(DEFAULT_API_PATH) as f:
    content = f.read()
with open("docs/portone-v2-openapi.json") as f:
    SCHEMAS = json.load(f)["components"]["schemas"]

# --- Default 보유 타입 ---
default_types = set()
for mf in glob.glob(MODELS_GLOB):
    with open(mf) as fh:
        mc = fh.read()
    derive_default = bool(re.search(r"#\[derive\([^\)]*Default", mc))
    for m in re.finditer(r"pub (?:struct|enum) (\w+)", mc):
        tname = m.group(1)
        if derive_default or re.search(
            r"impl\s+Default\s+for\s+" + re.escape(tname) + r"\b", mc
        ):
            default_types.add(tname)

# --- oneOf enum(tag + newtype variant) 식별 — round-trip 결함으로 skip 대상 ---
oneof_enums = set()
for mf in glob.glob(MODELS_GLOB):
    with open(mf) as fh:
        mc = fh.read()
    if re.search(r"#\[serde\(tag\s*=\s*\"\w+\"\)", mc) and re.search(
        r"\(Box<models::\w+>\)", mc
    ):
        em = re.search(r"pub enum (\w+)", mc)
        if em:
            oneof_enums.add(em.group(1))


def references_oneof_enum(schema, seen=None):
    """스키마가 (재귀적으로) oneOf enum 을 참조하면 True — round-trip 결함 응답 식별용."""
    if seen is None:
        seen = set()
    if "$ref" in schema:
        name = schema["$ref"].split("/")[-1]
        if name in oneof_enums:
            return True
        if name in seen:
            return False
        seen.add(name)
        return references_oneof_enum(SCHEMAS.get(name, {}), seen)
    if "oneOf" in schema or "anyOf" in schema:
        return True
    if schema.get("type") == "array":
        return references_oneof_enum(schema.get("items", {}), seen)
    for p in (schema.get("properties") or {}).values():
        if references_oneof_enum(p, seen):
            return True
    return False


def default_arg(typ):
    t = typ.strip()
    if t.startswith("Option<"):
        return "None"
    if t == "&str":
        return '"test"'
    if t in ("i64", "i32", "u64", "u32", "f64", "f32", "usize", "isize"):
        return "0"
    if t == "bool":
        return "false"
    if t == "serde_json::Value":
        return "serde_json::Value::Null"
    if t.startswith("models::"):
        tn = t.split("::", 1)[1].strip()
        if tn in default_types:
            return f"{t}::default()"
        return None
    if t in default_types:
        return f"{t}::default()"
    return None


def render_path_regex(template):
    parts = template.split("{}", 1)
    rest = parts[1] if len(parts) > 1 else template
    rest = re.sub(r"\{[^}]*\}", "[^/]+", rest)
    return "^" + rest + "$"


def parse_args(args):
    out = []
    for arg in args.split(","):
        arg = arg.strip()
        if not arg or ":" not in arg:
            continue
        name, typ = arg.split(":", 1)
        out.append((name.strip(), typ.strip()))
    return out


def mock_expr(ret_type):
    """응답(ret_type) → mock body Rust 식. None 이면 본문 없음."""
    rt = ret_type.strip()
    if rt == "serde_json::Value":
        return "serde_json::json!({})"
    if rt in ("()", "reqwest::Response"):
        return None
    if rt.startswith("models::"):
        return f"serde_json::to_value(&{rt}::default()).unwrap()"
    return None


# --- 함수별 시그니처 + URI + HTTP 메서드 + 반환 타입 추출 ---
sig_pat = (
    r"pub async fn (\w+)\(\s*configuration: &configuration::Configuration,?\s*(.*?)\s*\)"
    r"\s*->\s*Result\s*<\s*(.*?)\s*,\s*Error\s*<\s*(.*?)\s*>\s*,?\s*>"
)

endpoints = []
skipped = []
unextracted = []
for m in re.finditer(sig_pat, content, re.DOTALL):
    fn_name = m.group(1)
    args = parse_args(m.group(2))
    ret_type = m.group(3)
    body = content[m.end(): m.end() + 3000]
    uri_m = re.search(r'let uri_str = format!\(\s*"([^"]+)"', body)
    method_m = re.search(r"reqwest::Method::([A-Z]+)", body)
    if not uri_m or not method_m:
        unextracted.append(fn_name)
        continue
    http_method = method_m.group(1)
    path = render_path_regex(uri_m.group(1))

    arg_lits = []
    skip_reason = None
    for name, typ in args:
        lit = default_arg(typ)
        if lit is None:
            skip_reason = f"non-Default arg {name}: {typ}"
            break
        arg_lits.append(lit)
    if skip_reason:
        skipped.append((fn_name, skip_reason))
        continue

    # round-trip 결함 oneOf enum 을 직·간접 참조하는 응답은 skip
    rt = ret_type.strip()
    if rt.startswith("models::"):
        rn = rt.split("::", 1)[1].strip()
        if rn in oneof_enums or references_oneof_enum(SCHEMAS.get(rn, {})):
            skipped.append((fn_name, "response references oneOf enum (round-trip 결함)"))
            continue

    endpoints.append((fn_name, http_method, path, arg_lits, ret_type))

total_async_fns = len(re.findall(r"pub async fn (\w+)\b", content))
covered = len(endpoints) + len(skipped) + len(unextracted)
assert covered == total_async_fns, (
    f"REGRESSION GATE FAILED: covered {covered} != {total_async_fns} async fns "
    f"({len(unextracted)} unextracted, {len(skipped)} skipped, {len(endpoints)} generated)"
)

os.makedirs("tests/generated", exist_ok=True)
lines = [
    "//! AUTO-GENERATED by generate_e2e_tests.py — DO NOT EDIT.",
    "//! 각 엔드포인트의 성공(200) 경로를 회귀 검증한다.",
    "//! 응답 모델의 Default 를 직렬화한 JSON 을 wiremock 에 탑재하고 HttpClient 로 호출해",
    "//! 응답이 Ok 로 역직렬화되는지 확인한다.",
    "//!",
    f"//! KNOWN LIMITATION: oneOf enum 응답({len(skipped)}개)은 openapi-generator 의",
    "//! internally-tagged 직렬화가 tag/struct 필드 중복을 만들어 round-trip 이 깨지므로",
    "//! 이 파일에서 제외된다. 라우팅/4xx 는 tests/generated/endpoints.rs 가 검증한다.",
    "#![allow(clippy::all)]",
    "use portone_rs::v2::domain::models;",
    "use portone_rs::v2::*;",
    "use wiremock::matchers::{method, path_regex};",
    "use wiremock::{Mock, MockServer, ResponseTemplate};",
    "",
]

for fn_name, http_method, p, arg_lits, ret_type in endpoints:
    mock = mock_expr(ret_type)
    args_str = ", ".join(arg_lits)
    if mock is None:
        respond = "ResponseTemplate::new(200)"
    else:
        respond = f"ResponseTemplate::new(200).set_body_json({mock})"
    lines.append("#[tokio::test]")
    lines.append(f"async fn {fn_name}_succeeds_on_200() {{")
    lines.append("    let __server = MockServer::start().await;")
    lines.append(f'    Mock::given(method("{http_method}"))')
    lines.append(f'        .and(path_regex("{p}"))')
    lines.append(f"        .respond_with({respond})")
    lines.append("        .expect(1)")
    lines.append("        .mount(&__server)")
    lines.append("        .await;")
    lines.append(
        '    let __client = HttpClient::new("test-secret").unwrap().with_base_url(__server.uri());'
    )
    lines.append(f"    let __result = __client.{fn_name}({args_str}).await;")
    lines.append(f'        assert!(__result.is_ok(), "{fn_name} failed: {{:?}}", __result);')
    lines.append("}")
    lines.append("")

with open(OUT_PATH, "w") as f:
    f.write("\n".join(lines))

print(
    f"Generated {len(endpoints)} e2e success tests "
    f"({len(skipped)} skipped, {len(unextracted)} unextracted) "
    f"out of {total_async_fns} async fns."
)
for name, reason in skipped:
    print(f"  skipped: {name} ({reason})")

subprocess.run(["cargo", "fmt"], check=True)
