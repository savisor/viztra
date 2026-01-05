//! Balance Entries insight implementation
//! Returns entries where type == 2 AND entry == 0

mod params;
mod query;

use crate::features::insights::insight_trait::Insight;
use crate::shared::error::AppError;
use params::BalanceEntriesParams;
use query::execute_query;
use serde_json::{json, Value};

/// Insight that returns balance entries (type == 2 AND entry == 0)
pub struct BalanceEntriesInsight;

impl BalanceEntriesInsight {
    pub fn new() -> Self {
        Self
    }
}

impl Default for BalanceEntriesInsight {
    fn default() -> Self {
        Self::new()
    }
}

impl Insight for BalanceEntriesInsight {
    fn identifier(&self) -> &'static str {
        "deals.balance_entries"
    }

    fn name(&self) -> &'static str {
        "Balance Entries"
    }

    fn description(&self) -> &'static str {
        "Returns all deal entries where type == 2 AND entry == 0"
    }

    fn parameter_schema(&self) -> Value {
        // Generate JSON Schema from the params struct
        let schema = schemars::schema_for!(BalanceEntriesParams);
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
        let _: BalanceEntriesParams = serde_json::from_value(params.clone())
            .map_err(|e| AppError::new(format!("Invalid parameters: {}", e)))?;
        
        Ok(())
    }

    fn execute(&self, params: Value) -> Result<Value, AppError> {
        // Deserialize parameters
        let params: BalanceEntriesParams = serde_json::from_value(params)
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

