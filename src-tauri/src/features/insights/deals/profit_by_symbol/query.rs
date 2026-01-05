//! Polars query logic for profit_by_symbol insight

use crate::features::insights::deals::profit_by_symbol::params::ProfitBySymbolParams;
use crate::features::insights::deals::profit_by_symbol::result::ProfitBySymbolResult;
use crate::shared::error::AppError;
use crate::shared::utils::cache_dir;
use polars::prelude::*;
use std::fs;

/// Executes the profit_by_symbol query
pub fn execute_query(params: &ProfitBySymbolParams) -> Result<Vec<ProfitBySymbolResult>, AppError> {
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
        return Ok(Vec::new());
    }

    // Read and combine all parquet files
    let mut dataframes = Vec::new();
    for file_path in &files_to_read {
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
        .collect()
        .map_err(|e| AppError::new(format!("Failed to collect data from {}: {}", file_path.display(), e)))?;
        
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

    // Convert back to LazyFrame for query operations
    let mut query = combined
        .lazy()
        .group_by([col("symbol")])
        .agg([
            col("profit").sum().alias("total_profit"),
            col("volume").sum().alias("total_volume"),
            col("ticket").count().alias("trade_count"),
        ]);

    // Apply min_profit filter before collecting if specified
    if let Some(min_profit) = params.min_profit {
        query = query.filter(col("total_profit").gt_eq(lit(min_profit)));
    }

    // Execute the query
    let result = query
        .sort(["total_profit"], SortMultipleOptions::default().with_order_descending(true))
        .collect()
        .map_err(|e| AppError::new(format!("Failed to execute query: {}", e)))?;

    // Convert to result structs
    let symbol_col = result
        .column("symbol")
        .map_err(|e| AppError::new(format!("Failed to get 'symbol' column: {}", e)))?
        .str()
        .map_err(|e| AppError::new(format!("Failed to cast 'symbol' column: {}", e)))?;
    let profit_col = result
        .column("total_profit")
        .map_err(|e| AppError::new(format!("Failed to get 'total_profit' column: {}", e)))?
        .f64()
        .map_err(|e| AppError::new(format!("Failed to cast 'total_profit' column: {}", e)))?;
    let volume_col = result
        .column("total_volume")
        .map_err(|e| AppError::new(format!("Failed to get 'total_volume' column: {}", e)))?
        .f64()
        .map_err(|e| AppError::new(format!("Failed to cast 'total_volume' column: {}", e)))?;
    let count_col = result
        .column("trade_count")
        .map_err(|e| AppError::new(format!("Failed to get 'trade_count' column: {}", e)))?
        .u32()
        .map_err(|e| AppError::new(format!("Failed to cast 'trade_count' column: {}", e)))?;

    let num_rows = result.height();
    let mut results = Vec::with_capacity(num_rows);

    for i in 0..num_rows {
        let symbol = symbol_col.get(i).unwrap_or("").to_string();
        let total_profit = profit_col.get(i).unwrap_or(0.0);
        let total_volume = volume_col.get(i).unwrap_or(0.0);
        let trade_count = count_col.get(i).unwrap_or(0) as i64;
        let avg_profit = if trade_count > 0 {
            total_profit / trade_count as f64
        } else {
            0.0
        };

        results.push(ProfitBySymbolResult {
            symbol,
            total_profit,
            total_volume,
            trade_count,
            avg_profit,
        });
    }

    Ok(results)
}

