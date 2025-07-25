use super::GuardAction;
use async_graphql::dynamic::ResolverContext;
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum QueryOperation {
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

pub trait LifecycleHooksInterface: Send + Sync {
    fn entity_guard(
        &self,
        _ctx: &ResolverContext,
        _entity: &str,
        _action: QueryOperation,
    ) -> GuardAction {
        GuardAction::Allow
    }

    fn field_guard(
        &self,
        _ctx: &ResolverContext,
        _entity: &str,
        _field: &str,
        _action: QueryOperation,
    ) -> GuardAction {
        GuardAction::Allow
    }
}

pub struct DefaultLifecycleHook;

impl LifecycleHooksInterface for DefaultLifecycleHook {}
