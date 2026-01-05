//! Result structure for profit_by_symbol insight

use serde::{Deserialize, Serialize};

/// Result row for profit_by_symbol insight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitBySymbolResult {
    /// The symbol name
    pub symbol: String,
    /// Total profit for this symbol
    pub total_profit: f64,
    /// Total volume traded
    pub total_volume: f64,
    /// Number of trades
    pub trade_count: i64,
    /// Average profit per trade
    pub avg_profit: f64,
}

