/**
 * Validation utilities
 */

use crate::error::AppError;

/// Validates that a string is not empty
pub fn validate_non_empty(value: &str, field_name: &str) -> Result<(), AppError> {
    if value.trim().is_empty() {
        return Err(AppError::new(format!("{} cannot be empty", field_name)));
    }
    Ok(())
}

/// Validates string length
pub fn validate_length(
    value: &str,
    min: usize,
    max: usize,
    field_name: &str,
) -> Result<(), AppError> {
    let len = value.len();
    if len < min || len > max {
        return Err(AppError::new(format!(
            "{} must be between {} and {} characters",
            field_name, min, max
        )));
    }
    Ok(())
}


