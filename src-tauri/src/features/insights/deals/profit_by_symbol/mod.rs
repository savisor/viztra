//! Profit by Symbol insight implementation

mod params;
mod result;
mod query;

use crate::features::insights::insight_trait::Insight;
use crate::shared::error::AppError;
use params::ProfitBySymbolParams;
use query::execute_query;
use serde_json::{json, Value};

/// Insight that calculates total profit grouped by symbol
pub struct ProfitBySymbolInsight;

impl ProfitBySymbolInsight {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ProfitBySymbolInsight {
    fn default() -> Self {
        Self::new()
    }
}

impl Insight for ProfitBySymbolInsight {
    fn identifier(&self) -> &'static str {
        "deals.profit_by_symbol"
    }

    fn name(&self) -> &'static str {
        "Profit by Symbol"
    }

    fn description(&self) -> &'static str {
        "Calculates total profit, volume, and trade count grouped by symbol from deal data"
    }

    fn parameter_schema(&self) -> Value {
        // Generate JSON Schema from the params struct
        let schema = schemars::schema_for!(ProfitBySymbolParams);
        serde_json::to_value(schema).unwrap_or_else(|_| {
            // Fallback schema if serialization fails
            json!({
                "type": "object",
                "properties": {
                    "account_number": {
                        "type": "string",
                        "description": "Optional account number (filename without .parquet extension)"
                    },
                    "min_profit": {
                        "type": "number",
                        "description": "Optional minimum profit threshold to filter results"
                    }
                }
            })
        })
    }

    fn validate_parameters(&self, params: &Value) -> Result<(), AppError> {
        // Try to deserialize to validate structure
        let _: ProfitBySymbolParams = serde_json::from_value(params.clone())
            .map_err(|e| AppError::new(format!("Invalid parameters: {}", e)))?;
        
        Ok(())
    }

    fn execute(&self, params: Value) -> Result<Value, AppError> {
        // Deserialize parameters
        let params: ProfitBySymbolParams = serde_json::from_value(params)
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

