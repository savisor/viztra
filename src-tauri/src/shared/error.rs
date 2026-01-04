//! Application error types

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppError {
    pub message: String,
    pub code: Option<String>,
}

impl AppError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: None,
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppError {}
