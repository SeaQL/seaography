use async_graphql::{
    dynamic::{Field, FieldFuture, FieldValue, InputValue, TypeRef},
    Error,
};
use heck::ToLowerCamelCase;
use sea_orm::{
    DatabaseConnection, EntityTrait, Iterable, JoinType, QueryFilter, QuerySelect, RelationTrait,
};

#[cfg(not(feature = "offset-pagination"))]
use crate::ConnectionObjectBuilder;
use crate::{
    apply_order, apply_pagination, get_filter_conditions, BuilderContext, EntityObjectBuilder,
    FilterInputBuilder, GuardAction, OrderInputBuilder, PaginationInputBuilder,
};

use super::get_cascade_conditions;

/// The configuration structure for EntityQueryFieldBuilder
pub struct EntityQueryFieldConfig {
    /// used to format entity field name
    pub type_name: crate::SimpleNamingFn,
    /// name for 'filters' field
    pub filters: String,
    /// name for 'orderBy' field
    pub order_by: String,
    /// name for 'pagination' field
    pub pagination: String,
}

impl std::default::Default for EntityQueryFieldConfig {
    fn default() -> Self {
        EntityQueryFieldConfig {
            type_name: Box::new(|object_name: &str| -> String {
                object_name.to_lower_camel_case()
            }),
            filters: "filter".into(),
            order_by: "orderBy".into(),
            pagination: "pagination".into(),
        }
    }
}

/// This builder produces a field for the Query object that queries a SeaORM entity
pub struct EntityQueryFieldBuilder {
    pub context: &'static BuilderContext,
}

impl EntityQueryFieldBuilder {
    /// used to get field name for a SeaORM entity
    pub fn type_name<T>(&self) -> String
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let entity_object = EntityObjectBuilder {
            context: self.context,
        };
        let object_name = entity_object.type_name::<T>();
        self.context.entity_query_field.type_name.as_ref()(&object_name)
    }

    /// used to get the Query object field for a SeaORM entity
    pub fn to_field<T>(&self) -> Field
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        #[cfg(not(feature = "offset-pagination"))]
        let connection_object_builder = ConnectionObjectBuilder {
            context: self.context,
        };
        let filter_input_builder = FilterInputBuilder {
            context: self.context,
        };
        let order_input_builder = OrderInputBuilder {
            context: self.context,
        };
        let pagination_input_builder = PaginationInputBuilder {
            context: self.context,
        };
        let entity_object = EntityObjectBuilder {
            context: self.context,
        };

        let object_name = entity_object.type_name::<T>();
        #[cfg(feature = "offset-pagination")]
        let type_ref = TypeRef::named_list(&object_name);
        #[cfg(not(feature = "offset-pagination"))]
        let type_ref = TypeRef::named_nn(connection_object_builder.type_name(&object_name));
        #[cfg(feature = "offset-pagination")]
        let resolver_fn =
            |object: Vec<T::Model>| FieldValue::list(object.into_iter().map(FieldValue::owned_any));
        #[cfg(not(feature = "offset-pagination"))]
        let resolver_fn = |object: crate::Connection<T>| FieldValue::owned_any(object);
        let guard = self.context.guards.entity_guards.get(&object_name);

        let context: &'static BuilderContext = self.context;

        Field::new(self.type_name::<T>(), type_ref, {
            move |ctx| {
                let context: &'static BuilderContext = context;
                FieldFuture::new({
                    async move {
                        let guard_flag = if let Some(guard) = guard {
                            (*guard)(&ctx)
                        } else {
                            GuardAction::Allow
                        };

                        if let GuardAction::Block(reason) = guard_flag {
                            return match reason {
                                Some(reason) => {
                                    Err::<Option<_>, async_graphql::Error>(Error::new(reason))
                                }
                                None => Err::<Option<_>, async_graphql::Error>(Error::new(
                                    "Entity guard triggered.",
                                )),
                            };
                        }

                        let filters = ctx.args.get(&context.entity_query_field.filters);
                        let filters = get_filter_conditions::<T>(context, filters);
                        let order_by = ctx.args.get(&context.entity_query_field.order_by);
                        let order_by = OrderInputBuilder { context }.parse_object::<T>(order_by);
                        let pagination = ctx.args.get(&context.entity_query_field.pagination);
                        let pagination =
                            PaginationInputBuilder { context }.parse_object(pagination);
                        let cascades = ctx.args.get("cascade");
                        let cascades = get_cascade_conditions(cascades);
                        let stmt = T::Relation::iter().fold(T::find(), |stmt, related_table| {
                            let related_table_name = related_table.def().to_tbl;
                            match related_table_name {
                                sea_orm::sea_query::TableRef::Table(iden) => {
                                    if cascades.contains(&iden.to_string()) {
                                        stmt.join(JoinType::InnerJoin, related_table.def())
                                            .distinct()
                                    } else {
                                        stmt
                                    }
                                }
                                sea_orm::sea_query::TableRef::SchemaTable(_, iden) => {
                                    if cascades.contains(&iden.to_string()) {
                                        stmt.join(JoinType::InnerJoin, related_table.def())
                                            .distinct()
                                    } else {
                                        stmt
                                    }
                                }
                                sea_orm::sea_query::TableRef::DatabaseSchemaTable(_, _, iden) => {
                                    if cascades.contains(&iden.to_string()) {
                                        stmt.join(JoinType::InnerJoin, related_table.def())
                                            .distinct()
                                    } else {
                                        stmt
                                    }
                                }
                                _ => stmt,
                            }
                        });
                        let stmt = stmt.filter(filters);
                        let stmt = apply_order(stmt, order_by);

                        let db = ctx.data::<DatabaseConnection>()?;

                        let object = apply_pagination::<T>(db, stmt, pagination).await?;

                        Ok(Some(resolver_fn(object)))
                    }
                })
            }
        })
        .argument(InputValue::new(
            "cascade",
            TypeRef::named_list(TypeRef::STRING),
        ))
        .argument(InputValue::new(
            &self.context.entity_query_field.filters,
            TypeRef::named(filter_input_builder.type_name(&object_name)),
        ))
        .argument(InputValue::new(
            &self.context.entity_query_field.order_by,
            TypeRef::named(order_input_builder.type_name(&object_name)),
        ))
        .argument(InputValue::new(
            &self.context.entity_query_field.pagination,
            TypeRef::named(pagination_input_builder.type_name()),
        ))
    }
}