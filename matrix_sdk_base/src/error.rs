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

//! Error conditions.

use std::io::Error as IoError;

use crate::api::Error as RumaClientError;
use crate::FromHttpResponseError as RumaResponseError;
use crate::IntoHttpError as RumaIntoHttpError;
use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;
use thiserror::Error;
use url::ParseError;

#[cfg(feature = "encryption")]
use matrix_sdk_crypto::{MegolmError, OlmError};

/// Result type of the rust-sdk.
pub type Result<T> = std::result::Result<T, Error>;

/// Internal representation of errors.
#[derive(Error, Debug)]
pub enum Error {
    /// Queried endpoint requires authentication but was called on an anonymous client.
    #[error("the queried endpoint requires authentication but was called before logging in")]
    AuthenticationRequired,
    /// An error at the HTTP layer.
    #[error(transparent)]
    Reqwest(#[from] ReqwestError),
    /// An error when parsing a string as a URI.
    #[error("can't parse the provided string as an URL")]
    Uri(#[from] ParseError),
    /// An error converting between ruma_client_api types and Hyper types.
    #[error("can't parse the JSON response as a Matrix response")]
    RumaResponse(RumaResponseError<RumaClientError>),
    /// An error converting between ruma_client_api types and Hyper types.
    #[error("can't convert between ruma_client_api and hyper types.")]
    IntoHttp(RumaIntoHttpError),
    /// An error de/serializing type for the `StateStore`
    #[error(transparent)]
    SerdeJson(#[from] JsonError),
    /// An error de/serializing type for the `StateStore`
    #[error(transparent)]
    IoError(#[from] IoError),
    #[cfg(feature = "encryption")]
    /// An error occurred during a E2EE operation.
    #[error(transparent)]
    OlmError(#[from] OlmError),
    /// An error occurred during a E2EE group operation.
    #[error(transparent)]
    MegolmError(#[from] MegolmError),
}

impl From<RumaResponseError<RumaClientError>> for Error {
    fn from(error: RumaResponseError<RumaClientError>) -> Self {
        Self::RumaResponse(error)
    }
}

impl From<RumaIntoHttpError> for Error {
    fn from(error: RumaIntoHttpError) -> Self {
        Self::IntoHttp(error)
    }
}
