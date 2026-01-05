//! Result structure for total_balance insight

use serde::{Deserialize, Serialize};

/// Result for total_balance insight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalBalanceResult {
    /// Total balance amount (sum of profit from balance entries)
    pub total_balance: f64,
}

