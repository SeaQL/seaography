use super::GuardAction;
use async_graphql::dynamic::ResolverContext;
use sea_orm::{entity::prelude::async_trait, Condition};
use std::{any::Any, ops::Deref};

pub struct LifecycleHooks(pub(crate) Box<dyn LifecycleHooksInterface>);

impl Default for LifecycleHooks {
    fn default() -> Self {
        Self(Box::new(DefaultLifecycleHook))
    }
}

#[derive(Default)]
pub struct MultiLifecycleHooks {
    hooks: Vec<Box<dyn LifecycleHooksInterface>>,
}

pub struct DefaultLifecycleHook;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OperationType {
    Read,
    Create,
    Update,
    Delete,
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

    /// This happens before an Entity is accessed, invoked on each field
    fn field_guard(
        &self,
        _ctx: &ResolverContext,
        _entity: &str,
        _field: &str,
        _action: OperationType,
    ) -> GuardAction {
        GuardAction::Allow
    }

    /// Apply custom filter to select, update and delete (but not insert)
    fn entity_filter(
        &self,
        _ctx: &ResolverContext,
        _entity: &str,
        _action: OperationType,
    ) -> Option<Condition> {
        None
    }

    /// Inspect and modify an ActiveModel before save (only insert for now)
    fn before_active_model_save(
        &self,
        _ctx: &ResolverContext,
        _entity: &str,
        _action: OperationType,
        _active_model: &mut dyn Any,
    ) -> GuardAction {
        GuardAction::Allow
    }
}

impl LifecycleHooksInterface for DefaultLifecycleHook {}

impl LifecycleHooks {
    pub fn new<T: LifecycleHooksInterface + 'static>(t: T) -> Self {
        Self(Box::new(t))
    }
}

impl Deref for LifecycleHooks {
    type Target = dyn LifecycleHooksInterface;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl MultiLifecycleHooks {
    #[allow(clippy::should_implement_trait)]
    pub fn add<T: LifecycleHooksInterface + 'static>(mut self, t: T) -> Self {
        self.hooks.push(Box::new(t));
        self
    }
}

#[async_trait::async_trait]
impl LifecycleHooksInterface for MultiLifecycleHooks {
    fn entity_guard(
        &self,
        ctx: &ResolverContext,
        entity: &str,
        action: OperationType,
    ) -> GuardAction {
        for hook in &self.hooks {
            let result = hook.entity_guard(ctx, entity, action);
            if matches!(result, GuardAction::Block(_)) {
                return result;
            }
        }
        GuardAction::Allow
    }

    async fn entity_watch(&self, ctx: &ResolverContext, entity: &str, action: OperationType) {
        for hook in &self.hooks {
            hook.entity_watch(ctx, entity, action).await;
        }
    }

    fn field_guard(
        &self,
        ctx: &ResolverContext,
        entity: &str,
        field: &str,
        action: OperationType,
    ) -> GuardAction {
        for hook in &self.hooks {
            let result = hook.field_guard(ctx, entity, field, action);
            if matches!(result, GuardAction::Block(_)) {
                return result;
            }
        }
        GuardAction::Allow
    }

    fn entity_filter(
        &self,
        ctx: &ResolverContext,
        entity: &str,
        action: OperationType,
    ) -> Option<Condition> {
        let mut cond = Condition::all();
        for hook in &self.hooks {
            if let Some(inner_cond) = hook.entity_filter(ctx, entity, action) {
                if !inner_cond.is_empty() {
                    cond = cond.add(inner_cond);
                }
            }
        }
        if !cond.is_empty() {
            Some(cond)
        } else {
            None
        }
    }
}
