//! Cache directory utilities for OS-specific paths

use crate::shared::error::AppError;
use std::path::{Path, PathBuf};

/// Gets the OS-specific cache directory for the application
/// 
/// Returns:
/// - Windows: `%LOCALAPPDATA%\viztra`
/// - macOS: `~/Library/Caches/viztra`
/// - Linux: `~/.cache/viztra`
pub fn get_cache_dir() -> Result<PathBuf, AppError> {
    #[cfg(windows)]
    {
        // On Windows, use data_local_dir (LOCALAPPDATA) instead of cache_dir
        // This matches where the files are actually stored
        let local_data_dir = dirs::data_local_dir()
            .ok_or_else(|| AppError::new("Failed to determine local data directory for this OS"))?;
        Ok(local_data_dir.join("viztra"))
    }
    
    #[cfg(not(windows))]
    {
        let cache_dir = dirs::cache_dir()
            .ok_or_else(|| AppError::new("Failed to determine cache directory for this OS"))?;
        Ok(cache_dir.join("viztra"))
    }
}

/// Gets the asset repository cache directory
pub fn get_asset_cache_dir() -> Result<PathBuf, AppError> {
    let cache_dir = get_cache_dir()?;
    Ok(cache_dir.join("assets"))
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
