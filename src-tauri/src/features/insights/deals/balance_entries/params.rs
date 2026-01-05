//! Parameters for balance_entries insight

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for the balance_entries insight
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BalanceEntriesParams {
    /// Optional account number (filename without .parquet extension)
    /// If not provided, will read from all deal files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
}

impl Default for BalanceEntriesParams {
    fn default() -> Self {
        Self {
            account_number: None,
        }
    }
}

