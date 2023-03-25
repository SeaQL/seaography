use async_graphql::dynamic::{Field, FieldFuture, FieldValue, Object, TypeRef};
use async_graphql::Value;
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

pub struct EdgeObjectConfig {
    pub type_name: Box<dyn Fn(&str) -> String + Sync + Send>,
    pub cursor: String,
    pub node: String,
}

impl std::default::Default for EdgeObjectConfig {
    fn default() -> EdgeObjectConfig {
        EdgeObjectConfig {
            type_name: Box::new(|object_name: &str| -> String { format!("{}Edge", object_name) }),
            cursor: "cursor".into(),
            node: "node".into(),
        }
    }
}

pub struct EdgeObjectBuilder {
    pub context: &'static BuilderContext,
}

impl EdgeObjectBuilder {
    pub fn type_name(&self, object_name: &str) -> String {
        self.context.edge_object.type_name.as_ref()(object_name)
    }

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
