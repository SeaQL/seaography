use async_graphql::Value;
use async_graphql::dynamic::{Field, FieldFuture, FieldValue, Object, TypeRef};
use sea_orm::EntityTrait;

use crate::{BuilderContext, EntityObjectBuilder};
/// used to represent a data Edge for GraphQL pagination
#[derive(Clone, Debug)]
pub struct Edge<T>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    /// cursor string
    pub cursor: String,

    /// data
    pub node: T::Model,
}

/// The configuration structure for EdgeObjectBuilder
pub struct EdgeObjectConfig {
    /// used to format the type name of the object
    pub type_name: crate::SimpleNamingFn,
    /// name for 'cursor' field
    pub cursor: String,
    /// name for 'node' field
    pub node: String,
}

impl std::default::Default for EdgeObjectConfig {
    fn default() -> EdgeObjectConfig {
        EdgeObjectConfig {
            type_name: Box::new(|object_name: &str| -> String { format!("{object_name}Edge") }),
            cursor: "cursor".into(),
            node: "node".into(),
        }
    }
}

/// This builder produces the Node object for a SeaORM entity
pub struct EdgeObjectBuilder {
    pub context: &'static BuilderContext,
}

impl EdgeObjectBuilder {
    /// used to get type name
    pub fn type_name(&self, object_name: &str) -> String {
        self.context.edge_object.type_name.as_ref()(object_name)
    }

    /// used to get the Node object for a SeaORM entity
    pub fn to_object<T>(&self) -> Object
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };
        let object_name = entity_object_builder.type_name::<T>();
        let name = self.type_name(&object_name);

        Object::new(name)
            .field(Field::new(
                &self.context.edge_object.cursor,
                TypeRef::named_nn(TypeRef::STRING),
                |ctx| {
                    FieldFuture::new(async move {
                        let edge = ctx.parent_value.try_downcast_ref::<Edge<T>>()?;
                        Ok(Some(Value::from(edge.cursor.as_str())))
                    })
                },
            ))
            .field(Field::new(
                &self.context.edge_object.node,
                TypeRef::named_nn(object_name),
                |ctx| {
                    FieldFuture::new(async move {
                        let edge = ctx.parent_value.try_downcast_ref::<Edge<T>>()?;
                        Ok(Some(FieldValue::borrowed_any(&edge.node)))
                    })
                },
            ))
    }
}
