use async_graphql::dynamic::*;
use sea_orm::prelude::*;

use crate::{edge::*, pagination::*};

/// used to represent a GraphQL Connection node for any Type
#[derive(Clone, Debug)]
pub struct Connection<T>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    /// cursor pagination info
    pub page_info: PageInfo,

    /// pagination info
    pub pagination_info: Option<PaginationInfo>,

    /// vector of data vector
    pub edges: Vec<Edge<T>>,
}

impl<T> Connection<T>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    pub fn entity_object_to_connection(entity_object: &Object, edge: &Object) -> Object {
        Object::new(format!("{}Connection", entity_object.type_name()))
            .field(Field::new(
                "pageInfo",
                TypeRef::named_nn(PageInfo::to_object().type_name()),
                |ctx| {
                    FieldFuture::new(async move {
                        let connection = ctx.parent_value.try_downcast_ref::<Connection<T>>()?;
                        Ok(Some(FieldValue::borrowed_any(&connection.page_info)))
                    })
                },
            ))
            .field(Field::new(
                "paginationInfo",
                TypeRef::named(PaginationInfo::to_object().type_name()),
                |ctx| {
                    FieldFuture::new(async move {
                        let connection = ctx.parent_value.try_downcast_ref::<Connection<T>>()?;
                        if let Some(value) = connection
                            .pagination_info
                            .as_ref()
                            .map(|v| FieldValue::borrowed_any(v))
                        {
                            Ok(Some(value))
                        } else {
                            Ok(FieldValue::NONE)
                        }
                    })
                },
            ))
            .field(Field::new(
                "nodes",
                TypeRef::named_nn_list_nn(entity_object.type_name()),
                |ctx| {
                    FieldFuture::new(async move {
                        let connection = ctx.parent_value.try_downcast_ref::<Connection<T>>()?;
                        Ok(Some(FieldValue::list(connection.edges.iter().map(
                            |edge: &Edge<T>| FieldValue::borrowed_any(&edge.node),
                        ))))
                    })
                },
            ))
            .field(Field::new(
                "edges",
                TypeRef::named_nn_list_nn(edge.type_name()),
                |ctx| {
                    FieldFuture::new(async move {
                        let connection = ctx.parent_value.try_downcast_ref::<Connection<T>>()?;
                        Ok(Some(FieldValue::list(
                            connection
                                .edges
                                .iter()
                                .map(|edge: &Edge<T>| FieldValue::borrowed_any(edge)),
                        )))
                    })
                },
            ))
    }
}
