//! Insights module - dynamic insight execution system

pub mod insight_trait;
pub mod model;
pub mod validator;
pub mod registry;
pub mod factory;
pub mod command;
pub mod batch;

// Re-export for convenience
pub use command::execute_insight;
pub use batch::execute_batch_insights;

// Insight implementations
pub mod deals;

