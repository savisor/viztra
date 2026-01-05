//! Deals command handlers

use crate::features::deals::model::{Deal, DealImportResult};
use crate::features::deals::service::DealService;

/// Tauri command: validate_and_store_deals
/// Validates and stores Parquet files for deals
/// 
/// # Arguments
/// * `files` - Vector of tuples (filename, file_data) where file_data is Vec<u8>
/// 
/// Returns DealImportResult with overall status and per-file messages
#[tauri::command]
pub fn validate_and_store_deals(
    files: Vec<(String, Vec<u8>)>,
) -> Result<DealImportResult, String> {
    DealService::validate_and_store_files(files)
        .map_err(|e| e.message)
}

/// Tauri command: read_deals_from_file
/// Reads deals from a single Parquet file
/// 
/// # Arguments
/// * `filename` - The filename of the Parquet file to read
/// 
/// Returns a vector of Deal records
#[tauri::command]
pub fn read_deals_from_file(filename: String) -> Result<Vec<Deal>, String> {
    DealService::read_deals_from_file(&filename)
        .map_err(|e| e.message)
}

/// Tauri command: read_all_deals
/// Reads all deals from all Parquet files in the deals cache directory
/// 
/// Returns a combined vector of all Deal records from all files
#[tauri::command]
pub fn read_all_deals() -> Result<Vec<Deal>, String> {
    DealService::read_all_deals()
        .map_err(|e| e.message)
}

