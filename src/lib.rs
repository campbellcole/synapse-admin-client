use std::fmt::Display;

use reqwest::{header::HeaderMap, Client};
use serde::Deserialize;
use thiserror::Error;
use url::ParseError;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_with;

/// https://matrix-org.github.io/synapse/latest/admin_api/account_validity.html
pub mod account_validity;
/// https://matrix-org.github.io/synapse/latest/usage/administration/admin_api/background_updates.html
pub mod background_updates;
/// https://matrix-org.github.io/synapse/latest/admin_api/event_reports.html
pub mod event_reports;
#[cfg(feature = "unstable-api")]
pub mod federation;
/// https://matrix-org.github.io/synapse/latest/admin_api/media_admin_api.html
pub mod media;
/// https://matrix-org.github.io/synapse/latest/admin_api/purge_history_api.html
pub mod purge_history;
/// https://matrix-org.github.io/synapse/latest/usage/administration/admin_api/registration_tokens.html
pub mod registration_tokens;
/// https://matrix-org.github.io/synapse/latest/admin_api/room_membership.html
pub mod room_membership;
/// https://matrix-org.github.io/synapse/latest/admin_api/rooms.html
pub mod rooms;
/// https://matrix-org.github.io/synapse/latest/admin_api/server_notices.html
pub mod server_notices;
/// https://matrix-org.github.io/synapse/latest/admin_api/version_api.html
pub mod server_version;
/// https://matrix-org.github.io/synapse/latest/admin_api/statistics.html
pub mod statistics;
/// https://matrix-org.github.io/synapse/latest/admin_api/user_admin_api.html
pub mod users;

mod prelude {
    pub use std::time::SystemTime;

    pub use matrix_sdk::ruma::{EventId, OwnedEventId, OwnedRoomId, OwnedUserId, RoomId, UserId};
    pub use serde_with::TimestampMilliSeconds;
    pub use thiserror::Error;
    pub use typed_builder::TypedBuilder;

    pub use crate::{endpoint, execute, MatrixResult, Result, SynapseClient};

    #[derive(Deserialize)]
    /// Quite a few endpoints return `{}` as a response, use this in those cases.
    pub struct EmptyObject {}

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub enum SortDirection {
        /// Oldest first
        #[serde(rename = "f")]
        Ascending,
        /// Newest first
        #[serde(rename = "b")]
        Descending,
    }
}

/// A reqwest client for the Synapse API.
#[derive(Debug, Clone)]
pub struct SynapseClient {
    inner: Client,
    api_url: String,
    api_port: u16,
}

/// An error encountered during a Synapse API request.
#[derive(Debug, Error)]
pub enum SynapseError {
    #[error("failed to build headers: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("failed to parse URL: {0}")]
    Parse(#[from] ParseError),
    #[error("request succeeded but matrix returned an error: {0}")]
    Matrix(#[from] MatrixError),
    #[error("request succeeded but response was not recognized: {0}")]
    UnknownResponse(serde_json::Value),
}

pub type Result<T> = ::std::result::Result<T, SynapseError>;

impl SynapseClient {
    pub fn new(api_url: String, api_port: u16, access_token: String) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.append("Authorization", format!("Bearer {access_token}").parse()?);
        let client = Client::builder().default_headers(headers).build()?;
        Ok(Self {
            inner: client,
            api_url,
            api_port,
        })
    }
}

/// A response from the matrix API.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MatrixResult<T> {
    Ok(T),
    Err(MatrixError),
    _Unknown(serde_json::Value),
}

/// An error returned by the matrix API.
#[derive(Debug, Deserialize, Error)]
pub struct MatrixError {
    #[serde(rename = "errcode")]
    pub code: String,
    #[serde(rename = "error")]
    pub message: String,
}

impl Display for MatrixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

#[macro_export]
macro_rules! execute {
    ($req:expr) => {
        match $req {
            MatrixResult::Ok(res) => Ok(res),
            MatrixResult::Err(err) => Err(err.into()),
            MatrixResult::_Unknown(res) => Err($crate::SynapseError::UnknownResponse(res)),
        }
    };
    ($req:expr; $res:ident => $value:expr) => {
        match $req {
            MatrixResult::Ok($res) => Ok($value),
            MatrixResult::Err(err) => Err(err.into()),
            MatrixResult::_Unknown(res) => Err($crate::SynapseError::UnknownResponse(res)),
        }
    };
}

#[macro_export]
macro_rules! endpoint {
    ($client:ident $path:expr) => {
        endpoint!(_inner $client "v1" $path)
    };
    (_inner $client:ident $version:literal $path:expr) => {
        format!(
            "{}:{}{}{}{}",
            $client.api_url, $client.api_port, "/_synapse/admin/", $version, $path,
        )
        .parse::<::reqwest::Url>()?
    };
    (v2 $client:ident $path:expr) => {
        endpoint!(_inner $client "v2" $path)
    };
}
