use sea_orm::{EntityName, EntityTrait, IdenStatic};
use std::borrow::Cow;

use crate::BuilderContext;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EntityColumnId {
    entity_name: Cow<'static, str>,
    column_name: Cow<'static, str>,
}

impl EntityColumnId {
    pub fn of<T>(column: &T::Column) -> EntityColumnId
    where
        T: EntityTrait,
    {
        EntityColumnId {
            entity_name: Cow::Borrowed(<T as EntityName>::table_name(&T::default())),
            column_name: Cow::Borrowed(column.as_str()),
        }
    }

    pub fn with_array(&self) -> Self {
        Self {
            entity_name: self.entity_name.clone(),
            column_name: Cow::Owned(format!("{}.array", self.column_name)),
        }
    }

    pub fn entity_name(&self, context: &'static BuilderContext) -> String {
        context.entity_object.type_name.as_ref()(&self.entity_name)
    }

    pub fn column_name(&self, context: &'static BuilderContext) -> String {
        let entity_name = self.entity_name(context);
        context.entity_object.column_name.as_ref()(&entity_name, &self.column_name)
    }
}

impl std::fmt::Display for EntityColumnId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}.{}", self.entity_name, self.column_name)
    }
}
