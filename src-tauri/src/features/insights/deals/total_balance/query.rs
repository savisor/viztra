//! Polars query logic for total_balance insight
//! Filters deals where type == 2 AND entry == 0, then sums the profit field

use crate::features::insights::deals::total_balance::params::TotalBalanceParams;
use crate::features::insights::deals::total_balance::result::TotalBalanceResult;
use crate::shared::error::AppError;
use crate::shared::utils::cache_dir;
use polars::prelude::*;
use std::fs;

/// Executes the total_balance query
/// Returns the sum of profit from balance entries (type == 2 AND entry == 0)
pub fn execute_query(params: &TotalBalanceParams) -> Result<TotalBalanceResult, AppError> {
    let deals_dir = cache_dir::get_deals_cache_dir()?;

    // Determine which files to read
    let files_to_read = if let Some(ref account_number) = params.account_number {
        // Read specific account file
        let filename = if account_number.ends_with(".parquet") {
            account_number.clone()
        } else {
            format!("{}.parquet", account_number)
        };
        let file_path = deals_dir.join(&filename);
        if !file_path.exists() {
            return Err(AppError::new(format!(
                "Deal file not found: {}",
                file_path.display()
            )));
        }
        vec![file_path]
    } else {
        // Read all parquet files in deals directory
        let mut files = Vec::new();
        if deals_dir.exists() {
            let entries = fs::read_dir(&deals_dir)
                .map_err(|e| AppError::new(format!("Failed to read deals directory: {}", e)))?;
            
            for entry in entries {
                let entry = entry.map_err(|e| AppError::new(format!("Failed to read directory entry: {}", e)))?;
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("parquet") {
                    files.push(path);
                }
            }
        }
        files
    };

    if files_to_read.is_empty() {
        return Ok(TotalBalanceResult { total_balance: 0.0 });
    }

    // Read parquet files, apply filter at query level (before collect)
    let mut dataframes = Vec::new();
    for file_path in &files_to_read {
        // Apply filter at LazyFrame level (query level) - this is like SQL WHERE clause
        // Filter: type == 2 AND entry == 0 (same as balance_entries)
        let df = LazyFrame::scan_parquet(
            file_path,
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
        .map_err(|e| AppError::new(format!("Failed to scan parquet file {}: {}", file_path.display(), e)))?
        // Apply filter at query level (before collect) - type == 2 AND entry == 0
        .filter(col("type").eq(lit(2)).and(col("entry").eq(lit(0))))
        .collect()
        .map_err(|e| AppError::new(format!("Failed to collect filtered data from {}: {}", file_path.display(), e)))?;
        
        dataframes.push(df);
    }

    // Combine all dataframes
    let combined = if dataframes.len() == 1 {
        dataframes.into_iter().next().unwrap()
    } else {
        let first = dataframes.remove(0);
        dataframes
            .into_iter()
            .fold(first, |acc, df| {
                acc.vstack(&df)
                    .map_err(|e| AppError::new(format!("Failed to combine frames: {}", e)))
                    .unwrap()
            })
    };

    // Aggregate: sum profit from balance entries
    let result = combined
        .lazy()
        .select([col("profit").sum().alias("total_balance")])
        .collect()
        .map_err(|e| AppError::new(format!("Failed to execute query: {}", e)))?;

    // Extract total balance
    let total_balance_col = result
        .column("total_balance")
        .map_err(|e| AppError::new(format!("Failed to get 'total_balance' column: {}", e)))?
        .f64()
        .map_err(|e| AppError::new(format!("Failed to cast 'total_balance' column to f64: {}", e)))?;

    let total_balance = if result.height() > 0 {
        total_balance_col.get(0).unwrap_or(0.0)
    } else {
        0.0
    };

    Ok(TotalBalanceResult { total_balance })
}

