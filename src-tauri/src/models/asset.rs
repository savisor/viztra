//! Asset-related data models

use serde::{Deserialize, Serialize};

/// Result of an asset operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetOperationResult {
    pub success: bool,
    pub message: String,
}

impl AssetOperationResult {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: message.into(),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
        }
    }
}

/// Configuration for asset repository
pub struct AssetRepoConfig {
    pub owner: String,
    pub repo: String,
    pub branch: String,
}

impl Default for AssetRepoConfig {
    fn default() -> Self {
        Self {
            owner: "savisor".to_string(),
            repo: "historical-asset-prices".to_string(),
            branch: "main".to_string(),
        }
    }
}

impl AssetRepoConfig {
    pub fn zip_url(&self) -> String {
        format!(
            "https://github.com/{}/{}/archive/refs/heads/{}.zip",
            self.owner, self.repo, self.branch
        )
    }

    pub fn api_base_url(&self) -> String {
        format!("https://api.github.com/repos/{}/{}", self.owner, self.repo)
    }
}

/// OHLC (Open, High, Low, Close) data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OHLCData {
    pub time: i64,    // Unix timestamp in seconds
    pub open: f64,    // Opening price
    pub high: f64,    // Highest price during the interval
    pub low: f64,     // Lowest price during the interval
    pub close: f64,   // Closing price
}

