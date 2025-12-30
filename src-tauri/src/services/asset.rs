//! Asset service - orchestrates asset download and management operations

use crate::error::AppError;
use crate::models::asset::{AssetOperationResult, AssetRepoConfig, OHLCData};
use crate::services::download::DownloadService;
use crate::services::file_cleanup::FileCleanupService;
use crate::services::parquet_reader::ParquetReaderService;
use crate::utils::cache_dir;
use crate::utils::zip_extract;
use std::path::PathBuf;
use std::fs;

/// Service for managing asset downloads and operations
pub struct AssetService;

impl AssetService {
    /// Pulls all assets from the repository
    /// Downloads the entire repo, extracts it, and cleans up unwanted files
    pub async fn pull_assets() -> Result<AssetOperationResult, AppError> {
        let config = AssetRepoConfig::default();
        let cache_dir = cache_dir::get_asset_cache_dir()?;

        // Remove existing directory if it exists
        if cache_dir.exists() {
            fs::remove_dir_all(&cache_dir)
                .map_err(|e| AppError::new(format!("Failed to remove existing directory: {}", e)))?;
        }

        // Ensure parent directory exists
        let parent_dir = cache_dir
            .parent()
            .ok_or_else(|| AppError::new("Invalid cache directory path"))?;
        cache_dir::ensure_cache_dir(parent_dir)?;

        // Create temporary ZIP file path
        let temp_dir = parent_dir.join("temp");
        cache_dir::ensure_cache_dir(&temp_dir)?;
        let zip_path = temp_dir.join("repo.zip");

        // Download repository as ZIP
        DownloadService::download_repo_zip(&config, &zip_path).await?;

        // Extract ZIP to cache directory
        zip_extract::extract_zip(&zip_path, &cache_dir)?;

        // Remove temporary ZIP file
        fs::remove_file(&zip_path)
            .map_err(|e| AppError::new(format!("Failed to remove temp file: {}", e)))?;

        // Clean up unwanted files (.md, .git)
        FileCleanupService::cleanup_directory(&cache_dir)?;

        Ok(AssetOperationResult::success(format!(
            "Successfully pulled all assets to {}",
            cache_dir.display()
        )))
    }

    /// Pulls assets for a specific symbol
    /// Downloads only the parquet files for the given symbol
    pub async fn pull_asset_by_symbol(symbol: &str) -> Result<AssetOperationResult, AppError> {
        // Validate symbol
        crate::utils::validation::validate_non_empty(symbol, "symbol")?;
        
        // Validate symbol format (no path separators)
        if symbol.contains('/') || symbol.contains('\\') {
            return Err(AppError::new("Symbol cannot contain path separators"));
        }

        let config = AssetRepoConfig::default();
        let symbol_dir = cache_dir::get_symbol_cache_dir(symbol)?;

        // Remove existing symbol directory if it exists
        if symbol_dir.exists() {
            fs::remove_dir_all(&symbol_dir)
                .map_err(|e| AppError::new(format!("Failed to remove existing directory: {}", e)))?;
        }

        // Ensure parent directory exists
        let parent_dir = symbol_dir
            .parent()
            .ok_or_else(|| AppError::new("Invalid symbol directory path"))?;
        cache_dir::ensure_cache_dir(parent_dir)?;

        // Get list of parquet files for this symbol
        let parquet_files = DownloadService::list_symbol_files(&config, symbol).await?;

        if parquet_files.is_empty() {
            return Ok(AssetOperationResult::error(format!(
                "No parquet files found for symbol: {}",
                symbol
            )));
        }

        // Download each parquet file
        for file_path in &parquet_files {
            let file_name = PathBuf::from(file_path)
                .file_name()
                .ok_or_else(|| AppError::new(format!("Invalid file path: {}", file_path)))?
                .to_string_lossy()
                .to_string();

            let output_path = symbol_dir.join(&file_name);

            DownloadService::download_file(&config, file_path, &output_path).await?;
        }

        Ok(AssetOperationResult::success(format!(
            "Successfully pulled {} parquet file(s) for symbol {} to {}",
            parquet_files.len(),
            symbol,
            symbol_dir.display()
        )))
    }

    /// Lists all available symbols (folder names) in the cache directory
    /// Returns a vector of symbol names as strings
    pub fn list_symbols() -> Result<Vec<String>, AppError> {
        let asset_dir = cache_dir::get_asset_cache_dir()?;

        // If the directory doesn't exist, return empty vector
        if !asset_dir.exists() {
            return Ok(Vec::new());
        }

        let mut symbols = Vec::new();

        // Read directory entries
        let entries = fs::read_dir(&asset_dir)
            .map_err(|e| AppError::new(format!("Failed to read cache directory: {}", e)))?;

        for entry in entries {
            let entry = entry
                .map_err(|e| AppError::new(format!("Failed to read directory entry: {}", e)))?;
            
            let path = entry.path();
            
            // Only include directories (symbol folders)
            if path.is_dir() {
                if let Some(symbol_name) = path.file_name() {
                    let symbol = symbol_name.to_string_lossy().to_string();
                    symbols.push(symbol);
                }
            }
        }

        // Sort symbols alphabetically
        symbols.sort();

        Ok(symbols)
    }

    /// Retrieves OHLC data for a specific symbol and timeframe
    /// Reads the parquet file from the cache directory
    pub fn retrieve_asset_ochl(
        symbol: &str,
        timeframe: &str,
    ) -> Result<Vec<OHLCData>, AppError> {
        // Validate inputs
        crate::utils::validation::validate_non_empty(symbol, "symbol")?;
        crate::utils::validation::validate_non_empty(timeframe, "timeframe")?;

        // Validate symbol format (no path separators)
        if symbol.contains('/') || symbol.contains('\\') {
            return Err(AppError::new("Symbol cannot contain path separators"));
        }

        // Validate timeframe format (no path separators, no extension)
        if timeframe.contains('/') || timeframe.contains('\\') || timeframe.contains('.') {
            return Err(AppError::new("Timeframe cannot contain path separators or dots"));
        }

        // Construct file path: cache_dir/historical-asset-prices/symbol/timeframe.parquet
        let symbol_dir = cache_dir::get_symbol_cache_dir(symbol)?;
        let file_path = symbol_dir.join(format!("{}.parquet", timeframe));

        // Check if file exists
        if !file_path.exists() {
            return Err(AppError::new(format!(
                "Parquet file not found: {}",
                file_path.display()
            )));
        }

        // Read parquet file
        ParquetReaderService::read_ochl_data(&file_path)
    }
}

