/// Deals feature module
pub mod command;
pub mod service;
pub mod model;
pub mod validator;

pub use command::{validate_and_store_deals, read_deals_from_file, read_all_deals};

