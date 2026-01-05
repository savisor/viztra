//! Insight command handlers for Tauri

use crate::features::insights::factory::get_insight;
use crate::features::insights::model::{InsightRequest, InsightResponse};
use crate::features::insights::validator::ParameterValidator;
use serde_json::Value;

/// Tauri command: execute_insight
/// Executes an insight with the provided parameters
/// 
/// # Arguments
/// * `request` - InsightRequest containing the insight_id and parameters
/// 
/// Returns InsightResponse with success status, data, error, and column names
#[tauri::command]
pub fn execute_insight(request: InsightRequest) -> Result<InsightResponse, String> {
    // Get the insight from the registry
    let insight = get_insight(&request.insight_id)
        .ok_or_else(|| format!("Insight '{}' not found", request.insight_id))?;

    // Validate parameters
    let schema = insight.parameter_schema();
    if let Err(e) = ParameterValidator::validate(&schema, &request.parameters) {
        return Ok(InsightResponse::error(format!("Parameter validation failed: {}", e.message)));
    }

    // Also use the insight's own validation if it has custom validation
    if let Err(e) = insight.validate_parameters(&request.parameters) {
        return Ok(InsightResponse::error(format!("Parameter validation failed: {}", e.message)));
    }

    // Execute the insight
    let result = match insight.execute(request.parameters) {
        Ok(data) => {
            // Extract column names from the result
            let columns = extract_columns(&data);
            InsightResponse::success(data, columns)
        }
        Err(e) => InsightResponse::error(e.message),
    };

    Ok(result)
}

/// Extracts column names from the result data
/// Assumes data is an array of objects, extracts keys from the first object
fn extract_columns(data: &Value) -> Vec<String> {
    match data {
        Value::Array(arr) => {
            if let Some(first_obj) = arr.first() {
                if let Value::Object(obj) = first_obj {
                    obj.keys().cloned().collect()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }
        }
        _ => Vec::new(),
    }
}

