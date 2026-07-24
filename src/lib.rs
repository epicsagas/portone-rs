//! # PortOne Rust SDK (V2)
//!
//! ⚠️ **비공식 (Unofficial) Rust SDK**
//! 본 라이브러리는 개인이 개발한 비공식 Rust SDK입니다.
//! 주식회사 코리아포트원(PortOne)의 공식 지원이나 관리를 받지 않는 독립적인 오픈소스 프로젝트입니다.
//! 공식 SDK 및 개발자 문서는 [포트원 개발자센터](https://developers.portone.io/)를 참고하세요.

#![allow(clippy::empty_docs)]

pub mod adapter;
pub mod domain;
pub mod error;
pub mod port;

pub mod v2 {
    pub use crate::adapter;
    pub use crate::domain;
    pub use crate::error;
    pub use crate::port;

    pub use crate::adapter::http_client::HttpClient;
    pub use crate::error::SdkError;
    pub use crate::port::*;
}

pub use v2::*;

/// 자주 사용하는 타입을 한 번에 가져오기 위한 프렐루드입니다.
///
/// ```rust,no_run
/// use portone_rs::prelude::*;
/// ```
pub mod prelude {
    pub use crate::adapter::http_client::HttpClient;
    pub use crate::error::SdkError;
    pub use crate::port::{
        B2bPort, BillingKeyPort, IdentityVerificationPort, MiscPort, PaymentPort, PlatformPort,
    };
}
