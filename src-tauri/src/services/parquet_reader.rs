//! Parquet file reading service using Polars

use crate::error::AppError;
use crate::models::asset::OHLCData;
use polars::prelude::*;
use std::path::PathBuf;

/// Service for reading parquet files
pub struct ParquetReaderService;

impl ParquetReaderService {
    /// Reads OHLC data from a parquet file
    /// Returns a vector of OHLCData points
    pub fn read_ochl_data(file_path: &PathBuf) -> Result<Vec<OHLCData>, AppError> {
        // Read parquet file using Polars
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
        .map_err(|e| AppError::new(format!("Failed to scan parquet file: {}", e)))?
        .select([
            col("time"),
            col("open"),
            col("high"),
            col("low"),
            col("close"),
        ])
        .sort(["time"], SortMultipleOptions::default())
        .collect()
        .map_err(|e| AppError::new(format!("Failed to collect data: {}", e)))?;

        // Extract columns
        let time_col = df
            .column("time")
            .map_err(|e| AppError::new(format!("Failed to get 'time' column: {}", e)))?
            .i64()
            .map_err(|e| AppError::new(format!("Failed to cast 'time' column to i64: {}", e)))?;
        let open_col = df
            .column("open")
            .map_err(|e| AppError::new(format!("Failed to get 'open' column: {}", e)))?
            .f64()
            .map_err(|e| AppError::new(format!("Failed to cast 'open' column to f64: {}", e)))?;
        let high_col = df
            .column("high")
            .map_err(|e| AppError::new(format!("Failed to get 'high' column: {}", e)))?
            .f64()
            .map_err(|e| AppError::new(format!("Failed to cast 'high' column to f64: {}", e)))?;
        let low_col = df
            .column("low")
            .map_err(|e| AppError::new(format!("Failed to get 'low' column: {}", e)))?
            .f64()
            .map_err(|e| AppError::new(format!("Failed to cast 'low' column to f64: {}", e)))?;
        let close_col = df
            .column("close")
            .map_err(|e| AppError::new(format!("Failed to get 'close' column: {}", e)))?
            .f64()
            .map_err(|e| AppError::new(format!("Failed to cast 'close' column to f64: {}", e)))?;

        // Convert to OHLCData
        let num_rows = df.height();
        let mut ochl_data = Vec::with_capacity(num_rows);

        for i in 0..num_rows {
            let time = time_col.get(i).unwrap_or(0);
            let open = open_col.get(i).unwrap_or(0.0);
            let high = high_col.get(i).unwrap_or(0.0);
            let low = low_col.get(i).unwrap_or(0.0);
            let close = close_col.get(i).unwrap_or(0.0);

            ochl_data.push(OHLCData {
                time,
                open,
                high,
                low,
                close,
            });
        }

        Ok(ochl_data)
    }
}
