//! Parameters for total_balance insight

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for the total_balance insight
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TotalBalanceParams {
    /// Optional account number (filename without .parquet extension)
    /// If not provided, will read from all deal files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
}

impl Default for TotalBalanceParams {
    fn default() -> Self {
        Self {
            account_number: None,
        }
    }
}

