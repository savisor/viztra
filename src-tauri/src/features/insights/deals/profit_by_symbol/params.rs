//! Parameters for profit_by_symbol insight

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for the profit_by_symbol insight
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProfitBySymbolParams {
    /// Optional account number (filename without .parquet extension)
    /// If not provided, will aggregate across all deal files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    
    /// Optional minimum profit threshold to filter results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_profit: Option<f64>,
}

impl Default for ProfitBySymbolParams {
    fn default() -> Self {
        Self {
            account_number: None,
            min_profit: None,
        }
    }
}

