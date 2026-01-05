//! Polars query logic for balance_entries insight
//! Filters deals where type == 2 AND entry == 0

use crate::features::deals::model::Deal;
use crate::features::insights::deals::balance_entries::params::BalanceEntriesParams;
use crate::shared::error::AppError;
use crate::shared::utils::cache_dir;
use polars::prelude::*;
use std::fs;

/// Executes the balance_entries query
/// Returns deals where type == 2 AND entry == 0
pub fn execute_query(params: &BalanceEntriesParams) -> Result<Vec<Deal>, AppError> {
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

    // Read parquet files, apply filter at query level (before collect)
    let mut dataframes = Vec::new();
    for file_path in &files_to_read {
        // Apply filter at LazyFrame level (query level) - this is like SQL WHERE clause
        // Filter: type == 2 AND entry == 0
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

    // Sort by time
    let df = combined
        .sort(["time"], SortMultipleOptions::default())
        .map_err(|e| AppError::new(format!("Failed to sort data: {}", e)))?;

    // Convert to Deal structs (reuse logic from trade_entries)
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

