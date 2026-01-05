//! Parquet validator for deals schema

use crate::shared::error::AppError;
use crate::features::deals::model::deals_schema;
use polars::prelude::*;
use std::path::Path;

/// Service for validating deals Parquet files
pub struct ParquetValidator;

impl ParquetValidator {
    /// Validates that a Parquet file matches the exact deals schema
    /// Checks:
    /// - File is valid Parquet format (not corrupted)
    /// - All required columns exist with correct names
    /// - All columns have correct data types
    /// - All columns are non-nullable (no null values)
    pub fn validate_deals_schema(file_path: &Path) -> Result<(), AppError> {
        // First, check if file exists
        if !file_path.exists() {
            return Err(AppError::new(format!(
                "File does not exist: {}",
                file_path.display()
            )));
        }

        // Try to read the parquet file - this validates it's not corrupted
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
        .map_err(|e| AppError::new(format!("Failed to scan parquet file (file may be corrupted): {}", e)))?
        .collect()
        .map_err(|e| AppError::new(format!("Failed to read parquet file: {}", e)))?;

        // Get the schema
        let schema = df.schema();

        // Check all required columns exist
        for &required_col in deals_schema::REQUIRED_COLUMNS {
            if !schema.contains(required_col) {
                return Err(AppError::new(format!(
                    "Missing required column: {}",
                    required_col
                )));
            }
        }

        // Check for extra columns (strict schema validation)
        let actual_columns: std::collections::HashSet<_> = schema
            .iter_names()
            .map(|s| s.as_str())
            .collect();
        let required_columns: std::collections::HashSet<_> = deals_schema::REQUIRED_COLUMNS
            .iter()
            .copied()
            .collect();

        for actual_col in &actual_columns {
            if !required_columns.contains(actual_col) {
                return Err(AppError::new(format!(
                    "Unexpected column found: {}. Schema must match exactly.",
                    actual_col
                )));
            }
        }

        // Validate column types and nullability
        for &col_name in deals_schema::REQUIRED_COLUMNS {
            // Get the field from schema - we already know it exists from earlier check
            // Use get_field which returns Option<Field>
            let field_opt = schema.get_field(col_name);
            let field = field_opt
                .ok_or_else(|| AppError::new(format!("Column {} not found in schema", col_name)))?;

            // Check data type
            let expected_type = deals_schema::get_column_type(col_name)
                .ok_or_else(|| AppError::new(format!("Unknown column type for: {}", col_name)))?;

            let actual_type = field.data_type();

            // Polars uses different representations, so we need to match carefully
            // Int64 maps to Int64
            // Float64 maps to Float64
            // String maps to String
            if !Self::types_match(&expected_type, actual_type) {
                return Err(AppError::new(format!(
                    "Column '{}' has incorrect type. Expected {:?}, found {:?}",
                    col_name, expected_type, actual_type
                )));
            }

            // Check for null values (non-nullable requirement)
            let null_count = df
                .column(col_name)
                .map_err(|e| AppError::new(format!("Failed to get column '{}': {}", col_name, e)))?
                .null_count();

            if null_count > 0 {
                return Err(AppError::new(format!(
                    "Column '{}' contains {} null value(s). All columns must be non-nullable.",
                    col_name, null_count
                )));
            }
        }

        Ok(())
    }

    /// Helper function to check if types match
    /// Handles Polars type system nuances
    fn types_match(expected: &DataType, actual: &DataType) -> bool {
        match (expected, actual) {
            (DataType::Int64, DataType::Int64) => true,
            (DataType::Float64, DataType::Float64) => true,
            (DataType::String, DataType::String) => true,
            _ => false,
        }
    }
}

