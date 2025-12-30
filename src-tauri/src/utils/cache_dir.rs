//! Cache directory utilities for OS-specific paths

use crate::error::AppError;
use std::path::{Path, PathBuf};

/// Gets the OS-specific cache directory for the application
/// 
/// Returns:
/// - Windows: `%LOCALAPPDATA%\viztra\cache`
/// - macOS: `~/Library/Caches/viztra`
/// - Linux: `~/.cache/viztra`
pub fn get_cache_dir() -> Result<PathBuf, AppError> {
    let cache_dir = dirs::cache_dir()
        .ok_or_else(|| AppError::new("Failed to determine cache directory for this OS"))?;
    
    Ok(cache_dir.join("viztra"))
}

/// Gets the asset repository cache directory
pub fn get_asset_cache_dir() -> Result<PathBuf, AppError> {
    let cache_dir = get_cache_dir()?;
    Ok(cache_dir.join("historical-asset-prices"))
}

/// Gets the cache directory for a specific symbol
pub fn get_symbol_cache_dir(symbol: &str) -> Result<PathBuf, AppError> {
    let asset_dir = get_asset_cache_dir()?;
    Ok(asset_dir.join(symbol))
}

/// Ensures the cache directory exists
pub fn ensure_cache_dir(path: &Path) -> Result<(), AppError> {
    std::fs::create_dir_all(path)
        .map_err(|e| AppError::new(format!("Failed to create cache directory: {}", e)))
}

