//! Insight registry for storing and retrieving insights

use crate::features::insights::insight_trait::Insight;
use std::collections::HashMap;
use std::sync::Arc;

/// Registry that holds all available insights
pub struct InsightRegistry {
    insights: HashMap<String, Arc<dyn Insight>>,
}

impl InsightRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            insights: HashMap::new(),
        }
    }

    /// Register an insight in the registry
    pub fn register<I: Insight + 'static>(&mut self, insight: I) {
        let identifier = insight.identifier().to_string();
        self.insights.insert(identifier, Arc::new(insight));
    }

    /// Get an insight by its identifier
    pub fn get(&self, identifier: &str) -> Option<Arc<dyn Insight>> {
        self.insights.get(identifier).cloned()
    }

    /// Get all registered insight identifiers
    #[allow(dead_code)]
    pub fn list_identifiers(&self) -> Vec<String> {
        self.insights.keys().cloned().collect()
    }

    /// Check if an insight exists
    #[allow(dead_code)]
    pub fn exists(&self, identifier: &str) -> bool {
        self.insights.contains_key(identifier)
    }
}

impl Default for InsightRegistry {
    fn default() -> Self {
        Self::new()
    }
}

