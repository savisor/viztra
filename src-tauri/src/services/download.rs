//! Download service for fetching assets from GitHub

use crate::error::AppError;
use crate::models::asset::AssetRepoConfig;
use serde::Deserialize;
use std::io::Write;
use std::path::PathBuf;

/// Service for downloading assets from GitHub
pub struct DownloadService;

impl DownloadService {
    /// Downloads the entire repository as a ZIP file
    pub async fn download_repo_zip(
        config: &AssetRepoConfig,
        output_path: &PathBuf,
    ) -> Result<(), AppError> {
        let url = config.zip_url();
        
        let response = reqwest::get(&url)
            .await
            .map_err(|e| AppError::new(format!("Failed to fetch repository: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::new(format!(
                "Failed to download repository: HTTP {}",
                response.status()
            )));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| AppError::new(format!("Failed to read response: {}", e)))?;

        let mut file = std::fs::File::create(output_path)
            .map_err(|e| AppError::new(format!("Failed to create output file: {}", e)))?;

        file.write_all(&bytes)
            .map_err(|e| AppError::new(format!("Failed to write file: {}", e)))?;

        Ok(())
    }

    /// Downloads a specific file from GitHub
    pub async fn download_file(
        config: &AssetRepoConfig,
        file_path: &str,
        output_path: &PathBuf,
    ) -> Result<(), AppError> {
        let url = format!(
            "https://raw.githubusercontent.com/{}/{}/{}/{}",
            config.owner, config.repo, config.branch, file_path
        );

        let response = reqwest::get(&url)
            .await
            .map_err(|e| AppError::new(format!("Failed to fetch file: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::new(format!(
                "Failed to download file: HTTP {}",
                response.status()
            )));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| AppError::new(format!("Failed to read response: {}", e)))?;

        // Ensure parent directory exists
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AppError::new(format!("Failed to create directory: {}", e)))?;
        }

        let mut file = std::fs::File::create(output_path)
            .map_err(|e| AppError::new(format!("Failed to create output file: {}", e)))?;

        file.write_all(&bytes)
            .map_err(|e| AppError::new(format!("Failed to write file: {}", e)))?;

        Ok(())
    }

    /// Gets the list of parquet files in a symbol folder from GitHub API
    /// Uses the Contents API to list files in the symbol directory
    pub async fn list_symbol_files(
        config: &AssetRepoConfig,
        symbol: &str,
    ) -> Result<Vec<String>, AppError> {
        #[derive(Deserialize)]
        struct GitHubContentItem {
            name: String,
            #[serde(rename = "type")]
            item_type: String,
            path: String,
        }

        let url = format!(
            "{}/contents/{}?ref={}",
            config.api_base_url(),
            symbol,
            config.branch
        );

        let response = reqwest::get(&url)
            .await
            .map_err(|e| AppError::new(format!("Failed to fetch file list: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::new(format!(
                "Failed to get file list: HTTP {}. Symbol '{}' may not exist.",
                response.status(),
                symbol
            )));
        }

        let contents: Vec<GitHubContentItem> = response
            .json()
            .await
            .map_err(|e| AppError::new(format!("Failed to parse response: {}", e)))?;

        let parquet_files: Vec<String> = contents
            .into_iter()
            .filter(|item| {
                item.item_type == "file" && item.name.ends_with(".parquet")
            })
            .map(|item| item.path)
            .collect();

        Ok(parquet_files)
    }
}

