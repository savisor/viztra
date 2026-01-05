//! Parameters for trade_entries_with_balance insight

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for the trade_entries_with_balance insight
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TradeEntriesWithBalanceParams {
    /// Optional account number (filename without .parquet extension)
    /// If not provided, will read from all deal files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
}

impl Default for TradeEntriesWithBalanceParams {
    fn default() -> Self {
        Self {
            account_number: None,
        }
    }
}

