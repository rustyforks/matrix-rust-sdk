// Copyright 2020 Damir Jelić
// Copyright 2020 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This crate implements a [Matrix](https://matrix.org/) client library.
//!
//! ##  Crate Feature Flags
//!
//! The following crate feature flags are available:
//!
//! * `encryption`: Enables end-to-end encryption support in the library.
//! * `sqlite-cryptostore`: Enables a SQLite based store for the encryption
//! keys. If this is disabled and `encryption` support is enabled the keys will
//! by default be stored only in memory and thus lost after the client is
//! destroyed.
#![deny(missing_docs)]

pub use matrix_sdk_base::{Error, EventEmitter, Result, Room, Session};
pub use matrix_sdk_base::{JsonStore, RoomState, StateStore};
pub use matrix_sdk_common::*;
pub use reqwest::header::InvalidHeaderValue;

#[cfg(feature = "encryption")]
pub use matrix_sdk_base::{Device, TrustState};

mod async_client;
mod request_builder;
pub use async_client::{AsyncClient, AsyncClientConfig, SyncSettings};
pub use request_builder::{MessagesRequestBuilder, RoomBuilder};

pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");
