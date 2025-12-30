//! Asset command handlers

use crate::models::asset::{AssetOperationResult, OHLCData};
use crate::services::asset::AssetService;

/// Tauri command: pull_assets
/// Downloads the entire historical-asset-prices repository
/// Returns success status and message
#[tauri::command]
pub async fn pull_assets() -> Result<AssetOperationResult, String> {
    AssetService::pull_assets()
        .await
        .map_err(|e| e.message)
}

/// Tauri command: pull_asset_by_symbol
/// Downloads parquet files for a specific symbol
/// 
/// # Arguments
/// * `symbol` - The symbol name (e.g., "EURUSD")
/// 
/// Returns success status and message
#[tauri::command]
pub async fn pull_asset_by_symbol(symbol: String) -> Result<AssetOperationResult, String> {
    AssetService::pull_asset_by_symbol(&symbol)
        .await
        .map_err(|e| e.message)
}

/// Tauri command: list_symbols
/// Lists all available symbols (folder names) in the cache directory
/// Returns a vector of symbol names
#[tauri::command]
pub fn list_symbols() -> Result<Vec<String>, String> {
    AssetService::list_symbols()
        .map_err(|e| e.message)
}

/// Tauri command: retrieve_asset_ochl
/// Retrieves OHLC data for a specific symbol and timeframe
/// 
/// # Arguments
/// * `symbol` - The symbol name (e.g., "EURUSD")
/// * `timeframe` - The timeframe (e.g., "1M")
/// 
/// Returns a vector of OHLCData points
#[tauri::command]
pub fn retrieve_asset_ochl(symbol: String, timeframe: String) -> Result<Vec<OHLCData>, String> {
    AssetService::retrieve_asset_ochl(&symbol, &timeframe)
        .map_err(|e| e.message)
}

