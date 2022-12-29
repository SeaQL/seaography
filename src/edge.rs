use async_graphql::{dynamic::*, Value};
use sea_orm::prelude::*;

#[derive(Clone, Debug)]
pub struct Edge<T>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    pub cursor: String,
    pub node: T::Model,
}

impl<T> Edge<T>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    pub fn to_object(entity_object: &Object) -> Object {
        let name = format!("{}Edge", entity_object.type_name());
        Object::new(name)
            .field(Field::new(
                "cursor",
                TypeRef::named_nn(TypeRef::STRING),
                |ctx| {
                    FieldFuture::new(async move {
                        let edge = ctx.parent_value.try_downcast_ref::<Edge<T>>()?;
                        Ok(Some(Value::from(edge.cursor.as_str())))
                    })
                },
            ))
            .field(Field::new(
                "node",
                TypeRef::named_nn(entity_object.type_name()),
                |ctx| {
                    FieldFuture::new(async move {
                        let edge = ctx.parent_value.try_downcast_ref::<Edge<T>>()?;
                        Ok(Some(FieldValue::borrowed_any(&edge.node)))
                    })
                },
            ))
    }
}
