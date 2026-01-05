//! Trade Entries insight implementation
//! Returns all deal entries where entry == 1 (trade entries)

mod params;
mod query;

use crate::features::insights::insight_trait::Insight;
use crate::shared::error::AppError;
use params::TradeEntriesParams;
use query::execute_query;
use serde_json::{json, Value};

/// Insight that returns trade entries (entry == 1)
pub struct TradeEntriesInsight;

impl TradeEntriesInsight {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TradeEntriesInsight {
    fn default() -> Self {
        Self::new()
    }
}

impl Insight for TradeEntriesInsight {
    fn identifier(&self) -> &'static str {
        "deals.trade_entries"
    }

    fn name(&self) -> &'static str {
        "Trade Entries"
    }

    fn description(&self) -> &'static str {
        "Returns all deal entries where entry == 1 (trade entries)"
    }

    fn parameter_schema(&self) -> Value {
        // Generate JSON Schema from the params struct
        let schema = schemars::schema_for!(TradeEntriesParams);
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
        let _: TradeEntriesParams = serde_json::from_value(params.clone())
            .map_err(|e| AppError::new(format!("Invalid parameters: {}", e)))?;
        
        Ok(())
    }

    fn execute(&self, params: Value) -> Result<Value, AppError> {
        // Deserialize parameters
        let params: TradeEntriesParams = serde_json::from_value(params)
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

