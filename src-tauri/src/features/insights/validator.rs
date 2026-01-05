//! JSON Schema validation for insight parameters

use crate::shared::error::AppError;
use jsonschema::{Draft, JSONSchema};
use serde_json::Value;

/// Validates parameters against a JSON Schema
pub struct ParameterValidator;

impl ParameterValidator {
    /// Validates the provided parameters against the given JSON Schema
    /// Returns Ok(()) if valid, Err(AppError) with detailed validation errors if invalid
    pub fn validate(schema: &Value, params: &Value) -> Result<(), AppError> {
        // Compile the schema
        let compiled_schema = JSONSchema::options()
            .with_draft(Draft::Draft7)
            .compile(schema)
            .map_err(|e| AppError::new(format!("Invalid JSON Schema: {}", e)))?;

        // Validate the parameters
        let validation_result = compiled_schema.validate(params);

        match validation_result {
            Ok(_) => Ok(()),
            Err(errors) => {
                // Collect all validation errors into a detailed message
                let error_messages: Vec<String> = errors
                    .map(|error| {
                        let instance_path = error.instance_path.to_string();
                        if !instance_path.is_empty() {
                            format!("Parameter '{}': {}", instance_path, error)
                        } else {
                            error.to_string()
                        }
                    })
                    .collect();

                let error_message = if error_messages.len() == 1 {
                    error_messages[0].clone()
                } else {
                    format!("Validation failed with {} errors:\n{}", 
                        error_messages.len(),
                        error_messages.join("\n"))
                };

                Err(AppError::new(error_message))
            }
        }
    }
}

