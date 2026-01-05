//! Total Balance insight implementation
//! Sums the profit field from balance entries (type == 2 AND entry == 0)

mod params;
mod result;
mod query;

use crate::features::insights::insight_trait::Insight;
use crate::shared::error::AppError;
use params::TotalBalanceParams;
use query::execute_query;
use serde_json::{json, Value};

/// Insight that returns the total balance (sum of profit from balance entries)
pub struct TotalBalanceInsight;

impl TotalBalanceInsight {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TotalBalanceInsight {
    fn default() -> Self {
        Self::new()
    }
}

impl Insight for TotalBalanceInsight {
    fn identifier(&self) -> &'static str {
        "deals.total_balance"
    }

    fn name(&self) -> &'static str {
        "Total Balance"
    }

    fn description(&self) -> &'static str {
        "Returns the sum of profit from balance entries (type == 2 AND entry == 0)"
    }

    fn parameter_schema(&self) -> Value {
        // Generate JSON Schema from the params struct
        let schema = schemars::schema_for!(TotalBalanceParams);
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
        let _: TotalBalanceParams = serde_json::from_value(params.clone())
            .map_err(|e| AppError::new(format!("Invalid parameters: {}", e)))?;
        
        Ok(())
    }

    fn execute(&self, params: Value) -> Result<Value, AppError> {
        // Deserialize parameters
        let params: TotalBalanceParams = serde_json::from_value(params)
            .map_err(|e| AppError::new(format!("Failed to parse parameters: {}", e)))?;

        // Execute the query
        let result = execute_query(&params)?;

        // Serialize result to JSON and wrap in array for consistency
        let json_result = serde_json::to_value(result)
            .map_err(|e| AppError::new(format!("Failed to serialize result: {}", e)))?;
        
        // Wrap in array for consistency with other insights
        Ok(Value::Array(vec![json_result]))
    }
}

