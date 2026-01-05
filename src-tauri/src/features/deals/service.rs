//! Deals service - orchestrates deal import and validation operations

use crate::shared::error::AppError;
use crate::shared::utils::cache_dir;
use crate::features::deals::model::{Deal, DealImportResult, FileImportResult};
use crate::features::deals::validator::ParquetValidator;
use polars::prelude::*;
use std::path::Path;
use std::fs;

/// Service for managing deal imports and operations
pub struct DealService;

impl DealService {
    /// Validates and stores Parquet files for deals
    /// Accepts file data as Vec<u8> and original filenames
    /// For each file:
    /// - Validates schema using ParquetValidator
    /// - If valid, stores file in deals cache directory (keeping original name)
    /// Returns DealImportResult with overall status and per-file messages
    pub fn validate_and_store_files(
        files: Vec<(String, Vec<u8>)>,
    ) -> Result<DealImportResult, AppError> {
        if files.is_empty() {
            return Ok(DealImportResult::error(
                "No files provided".to_string(),
                Vec::new(),
            ));
        }

        // Get deals cache directory
        let deals_dir = cache_dir::get_deals_cache_dir()?;
        
        // Ensure cache directory exists
        cache_dir::ensure_cache_dir(&deals_dir)?;

        // Create temp directory for validation
        let temp_dir = deals_dir
            .parent()
            .ok_or_else(|| AppError::new("Invalid deals cache directory path"))?
            .join("temp");
        cache_dir::ensure_cache_dir(&temp_dir)?;

        let total_files = files.len();
        let mut file_results = Vec::new();
        let mut success_count = 0;
        let mut error_count = 0;

        // Process each file
        for (filename, file_data) in files {
            let file_result = Self::process_file(&deals_dir, &temp_dir, &filename, &file_data);
            
            match &file_result {
                FileImportResult { success: true, .. } => {
                    success_count += 1;
                }
                FileImportResult { success: false, .. } => {
                    error_count += 1;
                }
            }
            
            file_results.push(file_result);
        }

        // Clean up temp directory
        if temp_dir.exists() {
            let _ = fs::remove_dir_all(&temp_dir);
        }

        // Create overall result message
        let message = if error_count == 0 {
            format!(
                "Successfully imported {} file(s) to {}",
                success_count,
                deals_dir.display()
            )
        } else if success_count == 0 {
            format!("Failed to import all {} file(s)", total_files)
        } else {
            format!(
                "Imported {} file(s), {} file(s) failed",
                success_count, error_count
            )
        };

        let overall_success = error_count == 0;

        Ok(if overall_success {
            DealImportResult::success(message, file_results)
        } else {
            DealImportResult::error(message, file_results)
        })
    }

    /// Processes a single file: validates and stores it
    fn process_file(
        deals_dir: &Path,
        temp_dir: &Path,
        filename: &str,
        file_data: &[u8],
    ) -> FileImportResult {
        // Validate filename (sanitize)
        if filename.is_empty() {
            return FileImportResult::error(filename.to_string(), "Filename cannot be empty");
        }

        // Ensure filename ends with .parquet
        let sanitized_filename = if filename.ends_with(".parquet") {
            filename.to_string()
        } else {
            format!("{}.parquet", filename)
        };

        // Write file to temp location for validation
        let temp_file_path = temp_dir.join(&sanitized_filename);
        
        // Write file data to temp location
        if let Err(e) = fs::write(&temp_file_path, file_data) {
            return FileImportResult::error(
                filename.to_string(),
                format!("Failed to write file to temp location: {}", e),
            );
        }

        // Validate schema
        if let Err(e) = ParquetValidator::validate_deals_schema(&temp_file_path) {
            // Clean up temp file
            let _ = fs::remove_file(&temp_file_path);
            return FileImportResult::error(filename.to_string(), e.message);
        }

        // File is valid, move to deals directory
        let target_path = deals_dir.join(&sanitized_filename);

        // If file already exists, we could either overwrite or skip
        // For now, we'll overwrite (copy will replace if exists)
        if let Err(e) = fs::copy(&temp_file_path, &target_path) {
            let _ = fs::remove_file(&temp_file_path);
            return FileImportResult::error(
                filename.to_string(),
                format!("Failed to copy file to cache directory: {}", e),
            );
        }

        // Clean up temp file
        let _ = fs::remove_file(&temp_file_path);

        FileImportResult::success(
            filename.to_string(),
            format!("Successfully imported to {}", target_path.display()),
        )
    }

    /// Reads deals from a single Parquet file
    /// Returns a vector of Deal records
    pub fn read_deals_from_file(filename: &str) -> Result<Vec<Deal>, AppError> {
        let deals_dir = cache_dir::get_deals_cache_dir()?;
        
        // Ensure filename ends with .parquet
        let filename = if filename.ends_with(".parquet") {
            filename.to_string()
        } else {
            format!("{}.parquet", filename)
        };
        
        let file_path = deals_dir.join(&filename);
        
        if !file_path.exists() {
            return Err(AppError::new(format!(
                "File does not exist: {}",
                file_path.display()
            )));
        }

        // Read parquet file using Polars
        let df = LazyFrame::scan_parquet(
            &file_path,
            ScanArgsParquet {
                n_rows: None,
                cache: true,
                parallel: ParallelStrategy::Auto,
                rechunk: false,
                row_index: None,
                low_memory: false,
                cloud_options: None,
                use_statistics: true,
                hive_options: Default::default(),
                glob: false,
            },
        )
        .map_err(|e| AppError::new(format!("Failed to scan parquet file: {}", e)))?
        .collect()
        .map_err(|e| AppError::new(format!("Failed to collect data: {}", e)))?;

        // Extract all columns
        let ticket_col = df
            .column("ticket")
            .map_err(|e| AppError::new(format!("Failed to get 'ticket' column: {}", e)))?
            .i64()
            .map_err(|e| AppError::new(format!("Failed to cast 'ticket' column to i64: {}", e)))?;
        let order_col = df
            .column("order")
            .map_err(|e| AppError::new(format!("Failed to get 'order' column: {}", e)))?
            .i64()
            .map_err(|e| AppError::new(format!("Failed to cast 'order' column to i64: {}", e)))?;
        let time_col = df
            .column("time")
            .map_err(|e| AppError::new(format!("Failed to get 'time' column: {}", e)))?
            .i64()
            .map_err(|e| AppError::new(format!("Failed to cast 'time' column to i64: {}", e)))?;
        let time_msc_col = df
            .column("time_msc")
            .map_err(|e| AppError::new(format!("Failed to get 'time_msc' column: {}", e)))?
            .i64()
            .map_err(|e| AppError::new(format!("Failed to cast 'time_msc' column to i64: {}", e)))?;
        let type_col = df
            .column("type")
            .map_err(|e| AppError::new(format!("Failed to get 'type' column: {}", e)))?
            .i64()
            .map_err(|e| AppError::new(format!("Failed to cast 'type' column to i64: {}", e)))?;
        let entry_col = df
            .column("entry")
            .map_err(|e| AppError::new(format!("Failed to get 'entry' column: {}", e)))?
            .i64()
            .map_err(|e| AppError::new(format!("Failed to cast 'entry' column to i64: {}", e)))?;
        let magic_col = df
            .column("magic")
            .map_err(|e| AppError::new(format!("Failed to get 'magic' column: {}", e)))?
            .i64()
            .map_err(|e| AppError::new(format!("Failed to cast 'magic' column to i64: {}", e)))?;
        let position_id_col = df
            .column("position_id")
            .map_err(|e| AppError::new(format!("Failed to get 'position_id' column: {}", e)))?
            .i64()
            .map_err(|e| AppError::new(format!("Failed to cast 'position_id' column to i64: {}", e)))?;
        let reason_col = df
            .column("reason")
            .map_err(|e| AppError::new(format!("Failed to get 'reason' column: {}", e)))?
            .i64()
            .map_err(|e| AppError::new(format!("Failed to cast 'reason' column to i64: {}", e)))?;
        let volume_col = df
            .column("volume")
            .map_err(|e| AppError::new(format!("Failed to get 'volume' column: {}", e)))?
            .f64()
            .map_err(|e| AppError::new(format!("Failed to cast 'volume' column to f64: {}", e)))?;
        let price_col = df
            .column("price")
            .map_err(|e| AppError::new(format!("Failed to get 'price' column: {}", e)))?
            .f64()
            .map_err(|e| AppError::new(format!("Failed to cast 'price' column to f64: {}", e)))?;
        let commission_col = df
            .column("commission")
            .map_err(|e| AppError::new(format!("Failed to get 'commission' column: {}", e)))?
            .f64()
            .map_err(|e| AppError::new(format!("Failed to cast 'commission' column to f64: {}", e)))?;
        let swap_col = df
            .column("swap")
            .map_err(|e| AppError::new(format!("Failed to get 'swap' column: {}", e)))?
            .f64()
            .map_err(|e| AppError::new(format!("Failed to cast 'swap' column to f64: {}", e)))?;
        let profit_col = df
            .column("profit")
            .map_err(|e| AppError::new(format!("Failed to get 'profit' column: {}", e)))?
            .f64()
            .map_err(|e| AppError::new(format!("Failed to cast 'profit' column to f64: {}", e)))?;
        let fee_col = df
            .column("fee")
            .map_err(|e| AppError::new(format!("Failed to get 'fee' column: {}", e)))?
            .f64()
            .map_err(|e| AppError::new(format!("Failed to cast 'fee' column to f64: {}", e)))?;
        let symbol_col = df
            .column("symbol")
            .map_err(|e| AppError::new(format!("Failed to get 'symbol' column: {}", e)))?
            .str()
            .map_err(|e| AppError::new(format!("Failed to cast 'symbol' column to str: {}", e)))?;
        let comment_col = df
            .column("comment")
            .map_err(|e| AppError::new(format!("Failed to get 'comment' column: {}", e)))?
            .str()
            .map_err(|e| AppError::new(format!("Failed to cast 'comment' column to str: {}", e)))?;
        let external_id_col = df
            .column("external_id")
            .map_err(|e| AppError::new(format!("Failed to get 'external_id' column: {}", e)))?
            .str()
            .map_err(|e| AppError::new(format!("Failed to cast 'external_id' column to str: {}", e)))?;

        // Convert to Deal structs
        let num_rows = df.height();
        let mut deals = Vec::with_capacity(num_rows);

        for i in 0..num_rows {
            let deal = Deal {
                ticket: ticket_col.get(i).unwrap_or(0),
                order: order_col.get(i).unwrap_or(0),
                time: time_col.get(i).unwrap_or(0),
                time_msc: time_msc_col.get(i).unwrap_or(0),
                r#type: type_col.get(i).unwrap_or(0),
                entry: entry_col.get(i).unwrap_or(0),
                magic: magic_col.get(i).unwrap_or(0),
                position_id: position_id_col.get(i).unwrap_or(0),
                reason: reason_col.get(i).unwrap_or(0),
                volume: volume_col.get(i).unwrap_or(0.0),
                price: price_col.get(i).unwrap_or(0.0),
                commission: commission_col.get(i).unwrap_or(0.0),
                swap: swap_col.get(i).unwrap_or(0.0),
                profit: profit_col.get(i).unwrap_or(0.0),
                fee: fee_col.get(i).unwrap_or(0.0),
                symbol: symbol_col.get(i).unwrap_or("").to_string(),
                comment: comment_col.get(i).unwrap_or("").to_string(),
                external_id: external_id_col.get(i).unwrap_or("").to_string(),
            };
            deals.push(deal);
        }

        Ok(deals)
    }

    /// Reads all deals from all Parquet files in the deals cache directory
    /// Returns a combined vector of all Deal records
    pub fn read_all_deals() -> Result<Vec<Deal>, AppError> {
        let deals_dir = cache_dir::get_deals_cache_dir()?;
        
        // If directory doesn't exist, return empty vector
        if !deals_dir.exists() {
            return Ok(Vec::new());
        }

        let mut all_deals = Vec::new();

        // Read directory entries
        let entries = fs::read_dir(&deals_dir)
            .map_err(|e| AppError::new(format!("Failed to read deals directory: {}", e)))?;

        for entry in entries {
            let entry = entry
                .map_err(|e| AppError::new(format!("Failed to read directory entry: {}", e)))?;
            
            let path = entry.path();
            
            // Only process .parquet files
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    let file_name_str = file_name.to_string_lossy();
                    if file_name_str.ends_with(".parquet") {
                        // Read deals from this file
                        match Self::read_deals_from_file(&file_name_str) {
                            Ok(mut deals) => {
                                all_deals.append(&mut deals);
                            }
                            Err(e) => {
                                // Log error but continue processing other files
                                eprintln!("Warning: Failed to read file {}: {}", file_name_str, e.message);
                            }
                        }
                    }
                }
            }
        }

        Ok(all_deals)
    }
}

