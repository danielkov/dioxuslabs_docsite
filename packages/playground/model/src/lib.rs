use serde::{Deserialize, Serialize};
use std::string::FromUtf8Error;
use thiserror::Error;
use uuid::Uuid;

#[cfg(feature = "server")]
mod server;

#[cfg(feature = "web")]
mod web;

#[derive(Debug, Serialize, Deserialize)]
pub enum SocketMessage {
    BuildRequest(String),
    BuildFinished(Result<Uuid, String>),
    BuildStage(BuildStage),
    BuildDiagnostic(CargoDiagnostic),
    QueuePosition(usize),
    AlreadyConnected,
}

/// A stage of building from the playground.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BuildStage {
    Compiling {
        crates_compiled: usize,
        total_crates: usize,
        current_crate: String,
    },
    RunningBindgen,
    Other,
}

impl SocketMessage {
    pub fn as_json_string(&self) -> Result<String, SocketError> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TryFrom<String> for SocketMessage {
    type Error = SocketError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(serde_json::from_str(&value)?)
    }
}

/// A cargo diagnostic
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CargoDiagnostic {
    pub target_crate: String,
    pub level: CargoLevel,
    pub message: String,
    pub spans: Vec<CargoDiagnosticSpan>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CargoLevel {
    Error,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CargoDiagnosticSpan {
    pub is_primary: bool,
    pub line_start: usize,
    pub line_end: usize,
    pub column_start: usize,
    pub column_end: usize,
    pub label: Option<String>,
}

/// Any socket error.
#[derive(Debug, Error)]
pub enum SocketError {
    #[error(transparent)]
    ParseJson(#[from] serde_json::Error),

    #[error(transparent)]
    Utf8Decode(#[from] FromUtf8Error),

    #[cfg(feature = "web")]
    #[error(transparent)]
    Gloo(#[from] gloo_net::websocket::WebSocketError),

    #[cfg(feature = "server")]
    #[error(transparent)]
    Axum(#[from] axum::Error),
}
