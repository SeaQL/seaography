use async_graphql::{
    dataloader::DataLoader,
    dynamic::{Field, FieldFuture, FieldValue, InputValue, TypeRef, ValueAccessor},
    Error,
};
use heck::{ToLowerCamelCase, ToSnakeCase};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, Iden, ModelTrait, QueryFilter,
    Related, RelationDef,
};

#[cfg(not(feature = "offset-pagination"))]
use crate::ConnectionObjectBuilder;
use crate::{
    apply_memory_pagination, apply_order, apply_pagination, get_filter_conditions, BuilderContext,
    EntityObjectBuilder, FilterInputBuilder, GuardAction, HashableGroupKey, KeyComplex,
    NewOrderInputBuilder, OffsetInput, OneToManyLoader, OneToOneLoader, OrderInputBuilder,
    PageInput, PaginationInput, PaginationInputBuilder,
};

/// This builder produces a GraphQL field for an SeaORM entity related trait
/// that can be added to the entity object
pub struct EntityObjectViaRelationBuilder {
    pub context: &'static BuilderContext,
}

impl EntityObjectViaRelationBuilder {
    /// used to get a GraphQL field for an SeaORM entity related trait
    pub fn get_relation<T, R>(&self, name: &str) -> Field
    where
        T: Related<R>,
        T: EntityTrait,
        R: EntityTrait,
        <T as EntityTrait>::Model: Sync,
        <R as sea_orm::EntityTrait>::Model: Sync,
        <<T as sea_orm::EntityTrait>::Column as std::str::FromStr>::Err: core::fmt::Debug,
        <<R as sea_orm::EntityTrait>::Column as std::str::FromStr>::Err: core::fmt::Debug,
    {
        let name = if cfg!(feature = "field-snake-case") {
            name.to_snake_case()
        } else {
            name.to_lower_camel_case()
        };
        let context: &'static BuilderContext = self.context;
        let to_relation_definition = <T as Related<R>>::to();
        let (via_relation_definition, is_via_relation) = match <T as Related<R>>::via() {
            Some(def) => (def, true),
            None => (<T as Related<R>>::to(), false),
        };

        let entity_object_builder = EntityObjectBuilder { context };
        #[cfg(not(feature = "offset-pagination"))]
        let connection_object_builder = ConnectionObjectBuilder { context };
        let filter_input_builder = FilterInputBuilder { context };
        let order_input_builder = OrderInputBuilder { context };
        let new_order_input_builder = NewOrderInputBuilder { context };
        let object_name: String = entity_object_builder.type_name::<R>();
        #[cfg(feature = "offset-pagination")]
        let type_ref = TypeRef::named_list(&object_name);
        #[cfg(not(feature = "offset-pagination"))]
        let type_ref = TypeRef::named_nn(connection_object_builder.type_name(&object_name));

        #[cfg(feature = "offset-pagination")]
        let resolver_fn =
            |object: Vec<R::Model>| FieldValue::list(object.into_iter().map(FieldValue::owned_any));
        #[cfg(not(feature = "offset-pagination"))]
        let resolver_fn = |object: crate::Connection<R>| FieldValue::owned_any(object);
        let guard = self.context.guards.entity_guards.get(&object_name);

        let from_col = <T::Column as std::str::FromStr>::from_str(
            via_relation_definition
                .from_col
                .to_string()
                .to_snake_case()
                .as_str(),
        )
        .unwrap();

        let to_col = <R::Column as std::str::FromStr>::from_str(
            to_relation_definition
                .to_col
                .to_string()
                .to_snake_case()
                .as_str(),
        )
        .unwrap();

        let field = match via_relation_definition.is_owner {
            false => Field::new(name, TypeRef::named(&object_name), move |ctx| {
                FieldFuture::new(async move {
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

                    let parent: &T::Model = ctx
                        .parent_value
                        .try_downcast_ref::<T::Model>()
                        .expect("Parent should exist");

                    let loader = ctx.data_unchecked::<DataLoader<OneToOneLoader<R>>>();

                    let stmt = if <T as Related<R>>::via().is_some() {
                        <T as Related<R>>::find_related()
                    } else {
                        R::find()
                    };

                    let filters = ctx.args.get(&context.entity_query_field.filters);
                    let filters = get_filter_conditions::<R>(context, filters);
                    let order_by = ctx.args.get(&context.entity_query_field.order_by);
                    let mut order_by = OrderInputBuilder { context }.parse_object::<R>(order_by);
                    let order = ctx.args.get(&context.entity_query_field.order);
                    let order = NewOrderInputBuilder { context }.parse_object::<R>(order);
                    order_by.extend(order);
                    let key = KeyComplex::<R> {
                        key: vec![parent.get(from_col)],
                        meta: HashableGroupKey::<R> {
                            stmt,
                            columns: vec![to_col],
                            filters: Some(filters),
                            order_by,
                        },
                    };

                    let data = loader.load_one(key).await?;

                    if let Some(data) = data {
                        Ok(Some(FieldValue::owned_any(data)))
                    } else {
                        Ok(None)
                    }
                })
            }),
            true => Field::new(name, type_ref, move |ctx| {
                let context: &'static BuilderContext = context;
                FieldFuture::new(async move {
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

                    // FIXME: optimize union queries
                    // NOTE: each has unique query in order to apply pagination...
                    let parent: &T::Model = ctx
                        .parent_value
                        .try_downcast_ref::<T::Model>()
                        .expect("Parent should exist");

                    let stmt = if <T as Related<R>>::via().is_some() {
                        <T as Related<R>>::find_related()
                    } else {
                        R::find()
                    };

                    let filters = ctx.args.get(&context.entity_query_field.filters);
                    let filters = get_filter_conditions::<R>(context, filters);

                    let order_by = ctx.args.get(&context.entity_query_field.order_by);
                    let mut order_by = OrderInputBuilder { context }.parse_object::<R>(order_by);

                    let order = ctx.args.get(&context.entity_query_field.order);
                    let order = NewOrderInputBuilder { context }.parse_object::<R>(order);
                    order_by.extend(order);

                    let pagination = ctx.args.get(&context.entity_query_field.pagination);
                    let pagination = PaginationInputBuilder { context }.parse_object(pagination);
                    let first = ctx.args.get("first");
                    let pagination = match first {
                        Some(first_value) => match first_value.u64() {
                            Ok(first_num) => {
                                if let Some(offset) = pagination.offset {
                                    PaginationInput {
                                        offset: Some(OffsetInput {
                                            offset: offset.offset,
                                            limit: first_num,
                                        }),
                                        page: None,
                                        cursor: None,
                                    }
                                } else if let Some(page) = pagination.page {
                                    PaginationInput {
                                        offset: None,
                                        page: Some(PageInput {
                                            page: page.page,
                                            limit: first_num,
                                        }),
                                        cursor: None,
                                    }
                                } else {
                                    PaginationInput {
                                        offset: Some(OffsetInput {
                                            offset: 0,
                                            limit: first_num,
                                        }),
                                        page: None,
                                        cursor: None,
                                    }
                                }
                            }
                            _error => pagination,
                        },
                        None => pagination,
                    };
                    let db = ctx.data::<DatabaseConnection>()?;

                    let object = if is_via_relation {
                        // TODO optimize query
                        let condition = Condition::all().add(from_col.eq(parent.get(from_col)));

                        let stmt = stmt.filter(condition.add(filters));
                        let stmt = apply_order(stmt, order_by);
                        apply_pagination::<R>(db, stmt, pagination).await?
                    } else {
                        let loader = ctx.data_unchecked::<DataLoader<OneToManyLoader<R>>>();

                        let key = KeyComplex::<R> {
                            key: vec![parent.get(from_col)],
                            meta: HashableGroupKey::<R> {
                                stmt,
                                columns: vec![to_col],
                                filters: Some(filters),
                                order_by,
                            },
                        };

                        let values = loader.load_one(key).await?;
                        apply_memory_pagination::<R>(values, pagination)
                    };

                    Ok(Some(resolver_fn(object)))
                })
            }),
        };

        field
            .argument(InputValue::new(
                &context.entity_query_field.filters,
                TypeRef::named(filter_input_builder.type_name(&object_name)),
            ))
            .argument(InputValue::new(
                &context.entity_query_field.order_by,
                TypeRef::named(order_input_builder.type_name(&object_name)),
            ))
            .argument(InputValue::new(
                &self.context.entity_query_field.order,
                TypeRef::named(new_order_input_builder.type_name(&object_name)),
            ))
            .argument(InputValue::new(
                &context.entity_query_field.pagination,
                TypeRef::named(&context.pagination_input.type_name),
            ))
            .argument(InputValue::new("first", TypeRef::named(TypeRef::INT)))
    }

    pub fn joiin<T, R>(
        &self,
        relation_definition: RelationDef,
        filter: Option<ValueAccessor>,
    ) -> RelationDef
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
        <<T as sea_orm::EntityTrait>::Column as std::str::FromStr>::Err: core::fmt::Debug,
        R: EntityTrait,
        <R as sea_orm::EntityTrait>::Model: Sync,
        <<R as sea_orm::EntityTrait>::Column as std::str::FromStr>::Err: core::fmt::Debug,
    {
        let filters = get_filter_conditions::<R>(self.context, filter);
        relation_definition.on_condition(move |_left, _right| filters.to_owned())
    }
}