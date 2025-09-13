use super::GuardAction;
use async_graphql::dynamic::ResolverContext;
use sea_orm::{entity::prelude::async_trait, Condition};
use std::ops::Deref;

pub struct LifecycleHooks(pub(crate) Box<dyn LifecycleHooksInterface>);

impl Deref for LifecycleHooks {
    type Target = dyn LifecycleHooksInterface;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl Default for LifecycleHooks {
    fn default() -> Self {
        Self(Box::new(DefaultLifecycleHook))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OperationType {
    Read,
    Create,
    Update,
    Delete,
}

impl LifecycleHooks {
    pub fn new<T: LifecycleHooksInterface + 'static>(t: T) -> Self {
        Self(Box::new(t))
    }
}

#[async_trait::async_trait]
pub trait LifecycleHooksInterface: Send + Sync {
    /// This happens before an Entity is accessed
    fn entity_guard(
        &self,
        _ctx: &ResolverContext,
        _entity: &str,
        _action: OperationType,
    ) -> GuardAction {
        GuardAction::Allow
    }

    /// This happens after an Entity is mutated
    async fn entity_watch(&self, _ctx: &ResolverContext, _entity: &str, _action: OperationType) {}

    fn field_guard(
        &self,
        _ctx: &ResolverContext,
        _entity: &str,
        _field: &str,
        _action: OperationType,
    ) -> GuardAction {
        GuardAction::Allow
    }

    fn entity_filter(
        &self,
        _ctx: &ResolverContext,
        _entity: &str,
        _action: OperationType,
    ) -> Option<Condition> {
        None
    }
}

pub struct DefaultLifecycleHook;

impl LifecycleHooksInterface for DefaultLifecycleHook {}
