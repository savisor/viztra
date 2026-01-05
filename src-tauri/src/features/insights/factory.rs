//! Insight factory for creating and managing the insight registry

use crate::features::insights::registry::InsightRegistry;
use crate::features::insights::insight_trait::Insight;
use std::sync::Arc;
use std::sync::OnceLock;

/// Global insight registry (initialized once, accessed via get_registry)
static REGISTRY: OnceLock<InsightRegistry> = OnceLock::new();

/// Initialize the insight registry with all available insights
/// This should be called once during application startup
pub fn initialize_registry() -> &'static InsightRegistry {
    REGISTRY.get_or_init(|| {
        let mut registry = InsightRegistry::new();
        
        // Register all insights here
        registry.register(crate::features::insights::deals::profit_by_symbol::ProfitBySymbolInsight::new());
        registry.register(crate::features::insights::deals::trade_entries::TradeEntriesInsight::new());
        registry.register(crate::features::insights::deals::all_entries::AllEntriesInsight::new());
        registry.register(crate::features::insights::deals::trade_entries_with_balance::TradeEntriesWithBalanceInsight::new());
        registry.register(crate::features::insights::deals::balance_entries::BalanceEntriesInsight::new());
        registry.register(crate::features::insights::deals::total_balance::TotalBalanceInsight::new());
        
        registry
    })
}

/// Get the global insight registry
/// Panics if the registry has not been initialized
pub fn get_registry() -> &'static InsightRegistry {
    REGISTRY.get().expect("Insight registry not initialized. Call initialize_registry() first.")
}

/// Get an insight by identifier
pub fn get_insight(identifier: &str) -> Option<Arc<dyn Insight>> {
    get_registry().get(identifier)
}

