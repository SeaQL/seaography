use std::collections::BTreeMap;

use async_graphql::dynamic::ResolverContext;

pub struct GuardsConfig {
    pub entity_guards: BTreeMap<String, FnGuard>,
    pub field_guards: BTreeMap<String, FnGuard>,
}

pub type FnGuard = Box<dyn Fn(&ResolverContext) -> bool + Sync + Send>;

impl std::default::Default for GuardsConfig {
    fn default() -> Self {
        Self {
            entity_guards: BTreeMap::new(),
            field_guards: BTreeMap::new(),
        }
    }
}
