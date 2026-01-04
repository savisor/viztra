//! Greet service - business logic for greeting functionality

use crate::shared::error::AppError;
use crate::shared::utils::validation;

pub struct GreetService;

impl GreetService {
    /// Creates a greeting message
    pub fn create_greeting(name: &str) -> Result<String, AppError> {
        validation::validate_non_empty(name, "name")?;
        validation::validate_length(name, 1, 100, "name")?;

        Ok(format!("Hello, {}! You've been greeted from Rust!", name))
    }
}
