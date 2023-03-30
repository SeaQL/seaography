use std::collections::BTreeMap;

use async_graphql::dynamic::ResolverContext;

/// Entities and Field guards configuration.
/// The guards are used to control access to entities or fields.
pub struct GuardsConfig {
    /// entity guards are executed before accessing an entity
    pub entity_guards: BTreeMap<String, FnGuard>,
    /// field guards are executed before accessing an entity field
    pub field_guards: BTreeMap<String, FnGuard>,
}

/// guards are functions that receive the application context
pub type FnGuard = Box<dyn Fn(&ResolverContext) -> bool + Sync + Send>;

impl std::default::Default for GuardsConfig {
    fn default() -> Self {
        Self {
            entity_guards: BTreeMap::new(),
            field_guards: BTreeMap::new(),
        }
    }
}
