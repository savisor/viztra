//! Deals-related data models

use serde::{Deserialize, Serialize};

/// Result of a deal import operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DealImportResult {
    pub success: bool,
    pub message: String,
    pub file_results: Vec<FileImportResult>,
}

impl DealImportResult {
    pub fn success(message: impl Into<String>, file_results: Vec<FileImportResult>) -> Self {
        Self {
            success: true,
            message: message.into(),
            file_results,
        }
    }

    pub fn error(message: impl Into<String>, file_results: Vec<FileImportResult>) -> Self {
        Self {
            success: false,
            message: message.into(),
            file_results,
        }
    }
}

/// Result for a single file import operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileImportResult {
    pub filename: String,
    pub success: bool,
    pub message: String,
}

impl FileImportResult {
    pub fn success(filename: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            filename: filename.into(),
            success: true,
            message: message.into(),
        }
    }

    pub fn error(filename: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            filename: filename.into(),
            success: false,
            message: message.into(),
        }
    }
}

/// Deal record from Parquet file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deal {
    pub ticket: i64,
    pub order: i64,
    pub time: i64,
    pub time_msc: i64,
    pub r#type: i64,
    pub entry: i64,
    pub magic: i64,
    pub position_id: i64,
    pub reason: i64,
    pub volume: f64,
    pub price: f64,
    pub commission: f64,
    pub swap: f64,
    pub profit: f64,
    pub fee: f64,
    pub symbol: String,
    pub comment: String,
    pub external_id: String,
}

/// Expected schema for deals Parquet files
pub mod deals_schema {
    use polars::prelude::DataType;

    /// Required column names for deals schema
    pub const REQUIRED_COLUMNS: &[&str] = &[
        "ticket",
        "order",
        "time",
        "time_msc",
        "type",
        "entry",
        "magic",
        "position_id",
        "reason",
        "volume",
        "price",
        "commission",
        "swap",
        "profit",
        "fee",
        "symbol",
        "comment",
        "external_id",
    ];

    /// Get expected data type for a column
    pub fn get_column_type(column_name: &str) -> Option<DataType> {
        match column_name {
            "ticket" | "order" | "time" | "time_msc" | "type" | "entry" | "magic"
            | "position_id" | "reason" => Some(DataType::Int64),
            "volume" | "price" | "commission" | "swap" | "profit" | "fee" => {
                Some(DataType::Float64)
            }
            "symbol" | "comment" | "external_id" => Some(DataType::String),
            _ => None,
        }
    }
}
