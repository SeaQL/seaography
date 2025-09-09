use sea_orm::{EntityName, EntityTrait, IdenStatic};

use crate::BuilderContext;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EntityColumnId {
    entity_name: String,
    column_name: String,
}

impl EntityColumnId {
    pub fn of<T>(column: &T::Column) -> EntityColumnId
    where
        T: EntityTrait,
    {
        EntityColumnId {
            entity_name: <T as EntityName>::table_name(&T::default()).into(),
            column_name: column.as_str().into(),
        }
    }

    pub fn with_array(&self) -> Self {
        Self {
            entity_name: self.entity_name.clone(),
            column_name: format!("{}.array", self.column_name),
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
