//! Insight request and response models

use serde::{Deserialize, Serialize};

/// Request to execute an insight
#[derive(Debug, Clone, Deserialize)]
pub struct InsightRequest {
    /// The unique identifier of the insight to execute
    pub insight_id: String,
    /// The parameters for the insight (dynamic shape based on insight)
    pub parameters: serde_json::Value,
}

/// Response from executing an insight
#[derive(Debug, Clone, Serialize)]
pub struct InsightResponse {
    /// Whether the execution was successful
    pub success: bool,
    /// The result data (array of objects, one per row)
    pub data: Option<serde_json::Value>,
    /// Error message if execution failed
    pub error: Option<String>,
    /// Column names for table rendering (extracted from result data)
    pub columns: Vec<String>,
}

impl InsightResponse {
    /// Create a successful response
    pub fn success(data: serde_json::Value, columns: Vec<String>) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            columns,
        }
    }

    /// Create an error response
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            columns: Vec::new(),
        }
    }
}

/// Batch request to execute multiple insights
#[derive(Debug, Clone, Deserialize)]
pub struct BatchInsightRequest {
    /// List of insight requests to execute concurrently
    pub requests: Vec<InsightRequest>,
}

/// Individual result item in a batch response
#[derive(Debug, Clone, Serialize)]
pub struct BatchInsightItem {
    /// The insight identifier for this result
    pub insight_id: String,
    /// Whether the execution was successful
    pub success: bool,
    /// The result data (array of objects, one per row)
    pub data: Option<serde_json::Value>,
    /// Error message if execution failed
    pub error: Option<String>,
    /// Column names for table rendering (extracted from result data)
    pub columns: Vec<String>,
}

impl BatchInsightItem {
    /// Create a successful batch item
    pub fn success(insight_id: String, data: serde_json::Value, columns: Vec<String>) -> Self {
        Self {
            insight_id,
            success: true,
            data: Some(data),
            error: None,
            columns,
        }
    }

    /// Create an error batch item
    pub fn error(insight_id: String, error_message: String) -> Self {
        Self {
            insight_id,
            success: false,
            data: None,
            error: Some(error_message),
            columns: Vec::new(),
        }
    }
}

/// Batch response containing results from multiple insights
#[derive(Debug, Clone, Serialize)]
pub struct BatchInsightResponse {
    /// Results for each insight request (in the same order as requests)
    pub results: Vec<BatchInsightItem>,
}

