//! All Entries insight implementation
//! Returns all deal entries with no filter

mod params;
mod query;

use crate::features::insights::insight_trait::Insight;
use crate::shared::error::AppError;
use params::AllEntriesParams;
use query::execute_query;
use serde_json::{json, Value};

/// Insight that returns all deal entries (no filter)
pub struct AllEntriesInsight;

impl AllEntriesInsight {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AllEntriesInsight {
    fn default() -> Self {
        Self::new()
    }
}

impl Insight for AllEntriesInsight {
    fn identifier(&self) -> &'static str {
        "deals.all_entries"
    }

    fn name(&self) -> &'static str {
        "All Deal Entries"
    }

    fn description(&self) -> &'static str {
        "Returns all deal entries with no filter"
    }

    fn parameter_schema(&self) -> Value {
        // Generate JSON Schema from the params struct
        let schema = schemars::schema_for!(AllEntriesParams);
        serde_json::to_value(schema).unwrap_or_else(|_| {
            // Fallback schema if serialization fails
            json!({
                "type": "object",
                "properties": {
                    "account_number": {
                        "type": "string",
                        "description": "Optional account number (filename without .parquet extension)"
                    }
                }
            })
        })
    }

    fn validate_parameters(&self, params: &Value) -> Result<(), AppError> {
        // Try to deserialize to validate structure
        let _: AllEntriesParams = serde_json::from_value(params.clone())
            .map_err(|e| AppError::new(format!("Invalid parameters: {}", e)))?;
        
        Ok(())
    }

    fn execute(&self, params: Value) -> Result<Value, AppError> {
        // Deserialize parameters
        let params: AllEntriesParams = serde_json::from_value(params)
            .map_err(|e| AppError::new(format!("Failed to parse parameters: {}", e)))?;

        // Execute the query
        let results = execute_query(&params)?;

        // Serialize results to JSON
        let json_results: Vec<Value> = results
            .into_iter()
            .map(|r| serde_json::to_value(r).unwrap_or_else(|_| json!({})))
            .collect();

        Ok(Value::Array(json_results))
    }
}

