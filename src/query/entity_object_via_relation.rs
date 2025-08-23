use async_graphql::{
    dataloader::DataLoader,
    dynamic::{Field, FieldFuture, FieldValue, InputValue, TypeRef},
};
use heck::{ToLowerCamelCase, ToSnakeCase};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, Iden, ModelTrait, QueryFilter,
    QueryTrait, Related,
};

use crate::{
    apply_guard, apply_memory_pagination, apply_order, apply_pagination, get_filter_conditions,
    guard_error, pluralize_unique, BuilderContext, ConnectionObjectBuilder, DatabaseContext,
    EntityObjectBuilder, FilterInputBuilder, GuardAction, HashableGroupKey, KeyComplex,
    OneToManyLoader, OneToOneLoader, OperationType, OrderInputBuilder, PaginationInputBuilder,
    UserContext,
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
        <R as sea_orm::EntityTrait>::Model: Sync,
        <<T as sea_orm::EntityTrait>::Column as std::str::FromStr>::Err: core::fmt::Debug,
        <<R as sea_orm::EntityTrait>::Column as std::str::FromStr>::Err: core::fmt::Debug,
    {
        let to_relation_definition = <T as Related<R>>::to();
        let name_pp = if cfg!(feature = "field-snake-case") {
            name.to_snake_case()
        } else {
            name.to_lower_camel_case()
        };
        let name = pluralize_unique(
            &name_pp,
            matches!(
                to_relation_definition.rel_type,
                sea_orm::RelationType::HasMany
            ),
        );
        let context: &'static BuilderContext = self.context;
        let (via_relation_definition, is_via_relation) = match <T as Related<R>>::via() {
            Some(def) => (def, true),
            None => (<T as Related<R>>::to(), false),
        };

        let entity_object_builder = EntityObjectBuilder { context };
        let connection_object_builder = ConnectionObjectBuilder { context };
        let filter_input_builder = FilterInputBuilder { context };
        let order_input_builder = OrderInputBuilder { context };

        let object_name: String = entity_object_builder.type_name::<R>();
        let object_name_ = object_name.clone();
        let guard = self.context.guards.entity_guards.get(&object_name);
        let hooks = &self.context.hooks;

        let from_col = <T::Column as std::str::FromStr>::from_str(
            via_relation_definition
                .from_col
                .to_string()
                .to_snake_case()
                .as_str(),
        )
        .unwrap_or_else(|_| panic!("Illegal from_col: {:?}", via_relation_definition.from_col));

        let to_col = <R::Column as std::str::FromStr>::from_str(
            to_relation_definition
                .to_col
                .to_string()
                .to_snake_case()
                .as_str(),
        )
        .unwrap_or_else(|_| panic!("Illegal from_col: {:?}", to_relation_definition.to_col));

        let field = match via_relation_definition.is_owner {
            false => Field::new(name, TypeRef::named(&object_name), move |ctx| {
                let object_name = object_name.clone();
                FieldFuture::new(async move {
                    if let GuardAction::Block(reason) = apply_guard(&ctx, guard) {
                        return Err(guard_error(reason, "Entity guard triggered."));
                    }
                    if let GuardAction::Block(reason) =
                        hooks.entity_guard(&ctx, &object_name, OperationType::Read)
                    {
                        return Err(guard_error(reason, "Entity guard triggered."));
                    }

                    let db = ctx
                        .data::<DatabaseConnection>()?
                        .restricted(ctx.data_opt::<UserContext>())?;

                    let parent = match ctx.parent_value.try_downcast_ref::<T::Model>() {
                        Ok(parent) => parent,
                        Err(_) => match ctx.parent_value.try_downcast_ref::<Option<T::Model>>() {
                            Ok(Some(parent)) => parent,
                            _ => {
                                return Err(async_graphql::Error::new(format!(
                                    "Failed to downcast object to {object_name}"
                                )));
                            }
                        },
                    };

                    let loader = ctx.data_unchecked::<DataLoader<OneToOneLoader<R>>>();

                    let mut stmt = if <T as Related<R>>::via().is_some() {
                        <T as Related<R>>::find_related()
                    } else {
                        R::find()
                    };
                    if let Some(filter) =
                        hooks.entity_filter(&ctx, &object_name, OperationType::Read)
                    {
                        stmt = stmt.filter(filter);
                    }

                    db.user_can_run(stmt.as_query())?;

                    let filters = ctx.args.get(&context.entity_query_field.filters);
                    let filters = get_filter_conditions::<R>(context, filters)?;
                    let order_by = ctx.args.get(&context.entity_query_field.order_by);
                    let order_by = OrderInputBuilder { context }.parse_object::<R>(order_by)?;
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
            true => Field::new(
                name,
                TypeRef::named_nn(connection_object_builder.type_name(&object_name)),
                move |ctx| {
                    let object_name = object_name.clone();
                    FieldFuture::new(async move {
                        if let GuardAction::Block(reason) = apply_guard(&ctx, guard) {
                            return Err(guard_error(reason, "Entity guard triggered."));
                        }
                        if let GuardAction::Block(reason) =
                            hooks.entity_guard(&ctx, &object_name, OperationType::Read)
                        {
                            return Err(guard_error(reason, "Entity guard triggered."));
                        }

                        // FIXME: optimize union queries
                        // NOTE: each has unique query in order to apply pagination...

                        let parent = match ctx.parent_value.try_downcast_ref::<T::Model>() {
                            Ok(parent) => parent,
                            Err(_) => {
                                match ctx.parent_value.try_downcast_ref::<Option<T::Model>>() {
                                    Ok(Some(parent)) => parent,
                                    _ => {
                                        return Err(async_graphql::Error::new(format!(
                                            "Failed to downcast object to {object_name}"
                                        )));
                                    }
                                }
                            }
                        };

                        let mut stmt = if <T as Related<R>>::via().is_some() {
                            <T as Related<R>>::find_related()
                        } else {
                            R::find()
                        };
                        if let Some(filter) =
                            hooks.entity_filter(&ctx, &object_name, OperationType::Read)
                        {
                            stmt = stmt.filter(filter);
                        }

                        let filters = ctx.args.get(&context.entity_query_field.filters);
                        let filters = get_filter_conditions::<R>(context, filters)?;

                        let order_by = ctx.args.get(&context.entity_query_field.order_by);
                        let order_by = OrderInputBuilder { context }.parse_object::<R>(order_by)?;

                        let pagination = ctx.args.get(&context.entity_query_field.pagination);
                        let pagination =
                            PaginationInputBuilder { context }.parse_object(pagination)?;

                        let db = &ctx
                            .data::<DatabaseConnection>()?
                            .restricted(ctx.data_opt::<UserContext>())?;

                        let connection = if is_via_relation {
                            // TODO optimize query
                            let condition = Condition::all().add(from_col.eq(parent.get(from_col)));

                            let stmt = stmt.filter(condition.add(filters));
                            let stmt = apply_order(stmt, order_by);
                            apply_pagination::<R>(db, stmt, pagination).await?
                        } else {
                            let loader = ctx.data_unchecked::<DataLoader<OneToManyLoader<R>>>();

                            db.user_can_run(stmt.as_query())?;

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

                            apply_memory_pagination(values, pagination)
                        };

                        Ok(Some(FieldValue::owned_any(connection)))
                    })
                },
            ),
        };

        match via_relation_definition.is_owner {
            false => field,
            true => field
                .argument(InputValue::new(
                    &context.entity_query_field.filters,
                    TypeRef::named(filter_input_builder.type_name(&object_name_)),
                ))
                .argument(InputValue::new(
                    &context.entity_query_field.order_by,
                    TypeRef::named(order_input_builder.type_name(&object_name_)),
                ))
                .argument(InputValue::new(
                    &context.entity_query_field.pagination,
                    TypeRef::named(&context.pagination_input.type_name),
                )),
        }
    }
}
