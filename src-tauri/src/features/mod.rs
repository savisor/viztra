/// Feature modules registry
pub mod greet;
pub mod asset;
pub mod deals;
pub mod insights;

pub use greet::greet;
pub use asset::{
    pull_assets,
    pull_asset_by_symbol,
    list_symbols,
    retrieve_asset_ochl,
};
pub use deals::{validate_and_store_deals, read_deals_from_file, read_all_deals};
pub use insights::{execute_insight, execute_batch_insights};