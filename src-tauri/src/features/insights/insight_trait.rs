//! Insight trait definition

use crate::shared::error::AppError;
use serde_json::Value;

/// Trait that all insights must implement
/// Provides a common interface for insight execution, validation, and metadata
pub trait Insight: Send + Sync {
    /// Returns the unique identifier for this insight (e.g., "deals.profit_by_symbol")
    fn identifier(&self) -> &'static str;

    /// Returns the human-readable name of this insight
    #[allow(dead_code)]
    fn name(&self) -> &'static str;

    /// Returns a description of what this insight does
    #[allow(dead_code)]
    fn description(&self) -> &'static str;

    /// Returns the JSON Schema for parameter validation
    fn parameter_schema(&self) -> Value;

    /// Validates the provided parameters against the insight's schema
    /// Returns Ok(()) if valid, Err(AppError) if invalid
    fn validate_parameters(&self, params: &Value) -> Result<(), AppError>;

    /// Executes the insight with the provided parameters
    /// Returns the result as a JSON Value (array of objects representing rows)
    fn execute(&self, params: Value) -> Result<Value, AppError>;
}

