use async_graphql::{
    dataloader::DataLoader,
    dynamic::{Field, FieldFuture, FieldValue, InputValue, TypeRef},
};
use heck::{ToLowerCamelCase, ToSnakeCase};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, QueryTrait, RelationDef};

use crate::{
    apply_memory_pagination, get_filter_conditions, guard_error, loader_impl, pluralize_unique,
    BuilderContext, Connection, ConnectionObjectBuilder, DatabaseContext, EntityObjectBuilder,
    FilterInputBuilder, GuardAction, HashableGroupKey, KeyComplex, OneToManyLoader, OneToOneLoader,
    OperationType, OrderInputBuilder, PaginationInputBuilder, UserContext,
};

/// This builder produces a GraphQL field for an SeaORM entity relationship
/// that can be added to the entity object
pub struct EntityObjectRelationBuilder {
    pub context: &'static BuilderContext,
}

impl EntityObjectRelationBuilder {
    /// to be called by SeaORM
    pub fn get_relation_name<T, R>(&self, name: &str, relation_definition: RelationDef) -> String
    where
        T: EntityTrait,
        R: EntityTrait,
    {
        self.get_relation_name_for(name, &relation_definition)
    }

    fn get_relation_name_for(&self, name: &str, relation_definition: &RelationDef) -> String {
        let name_pp = &if cfg!(feature = "field-snake-case") {
            name.to_snake_case()
        } else {
            name.to_lower_camel_case()
        };
        pluralize_unique(
            name_pp,
            matches!(relation_definition.rel_type, sea_orm::RelationType::HasMany),
        )
    }

    /// used to get a GraphQL field for an SeaORM entity relationship
    pub fn get_relation<T, R>(&self, name: &str, relation_definition: RelationDef) -> Field
    where
        T: EntityTrait,
        <<T as sea_orm::EntityTrait>::Column as std::str::FromStr>::Err: core::fmt::Debug,
        R: EntityTrait,
        <R as sea_orm::EntityTrait>::Model: Sync,
        <<R as sea_orm::EntityTrait>::Column as std::str::FromStr>::Err: core::fmt::Debug,
    {
        let name = self.get_relation_name_for(name, &relation_definition);
        let context: &'static BuilderContext = self.context;
        let entity_object_builder = EntityObjectBuilder { context };
        let connection_object_builder = ConnectionObjectBuilder { context };
        let filter_input_builder = FilterInputBuilder { context };
        let order_input_builder = OrderInputBuilder { context };

        let parent_name: String = entity_object_builder.type_name::<T>();
        let object_name: String = entity_object_builder.type_name::<R>();
        let object_name_ = object_name.clone();
        let hooks = &self.context.hooks;
        let relation_definition_is_owner = relation_definition.is_owner;

        let field_name = name.clone();
        let field = match relation_definition.is_owner {
            false => Field::new(name, TypeRef::named(&object_name), move |ctx| {
                let object_name = object_name.clone();
                let parent_name = parent_name.clone();
                let field_name = field_name.clone();
                let relation_definition = relation_definition.clone();
                FieldFuture::new(async move {
                    if let GuardAction::Block(reason) =
                        hooks.entity_guard(&ctx, &object_name, OperationType::Read)
                    {
                        return Err(guard_error(reason, "Entity guard triggered."));
                    }
                    if let GuardAction::Block(reason) =
                        hooks.field_guard(&ctx, &parent_name, &field_name, OperationType::Read)
                    {
                        return Err(guard_error(reason, "Field guard triggered."));
                    }

                    let Ok(parent) = ctx.parent_value.try_downcast_ref::<T::Model>() else {
                        return Err(async_graphql::Error::new(format!(
                            "Failed to downcast object to {}",
                            entity_object_builder.type_name::<T>()
                        )));
                    };

                    let loader = ctx.data_unchecked::<DataLoader<OneToOneLoader<R>>>();

                    let mut stmt = R::find();
                    if let Some(filter) =
                        hooks.entity_filter(&ctx, &object_name, OperationType::Read)
                    {
                        stmt = stmt.filter(filter);
                    }

                    let db = ctx
                        .data::<DatabaseConnection>()?
                        .restricted(ctx.data_opt::<UserContext>())?;

                    db.user_can_run(stmt.as_query())?;

                    let filters = ctx.args.get(&context.entity_query_field.filters);
                    let filters = get_filter_conditions::<R>(context, filters)?;
                    let order_by = ctx.args.get(&context.entity_query_field.order_by);
                    let order_by = OrderInputBuilder { context }.parse_object::<R>(order_by)?;

                    let key = KeyComplex::<R> {
                        key: loader_impl::extract_key::<T::Model>(
                            &relation_definition.from_col,
                            parent,
                        )?,
                        meta: HashableGroupKey::<R> {
                            stmt,
                            junction_fields: Vec::new(),
                            rel_def: relation_definition,
                            via_def: None,
                            filters,
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
                    let parent_name = parent_name.clone();
                    let field_name = field_name.clone();
                    let relation_definition = relation_definition.clone();
                    let context: &'static BuilderContext = context;
                    FieldFuture::new(async move {
                        if let GuardAction::Block(reason) =
                            hooks.entity_guard(&ctx, &object_name, OperationType::Read)
                        {
                            return Err(guard_error(reason, "Entity guard triggered."));
                        }
                        if let GuardAction::Block(reason) =
                            hooks.field_guard(&ctx, &parent_name, &field_name, OperationType::Read)
                        {
                            return Err(guard_error(reason, "Field guard triggered."));
                        }

                        let Ok(parent) = ctx.parent_value.try_downcast_ref::<T::Model>() else {
                            return Err(async_graphql::Error::new(format!(
                                "Failed to downcast object to {}",
                                entity_object_builder.type_name::<T>()
                            )));
                        };

                        let loader = ctx.data_unchecked::<DataLoader<OneToManyLoader<R>>>();

                        let mut stmt = R::find();
                        if let Some(filter) =
                            hooks.entity_filter(&ctx, &object_name, OperationType::Read)
                        {
                            stmt = stmt.filter(filter);
                        }

                        let db = &ctx
                            .data::<DatabaseConnection>()?
                            .restricted(ctx.data_opt::<UserContext>())?;

                        db.user_can_run(stmt.as_query())?;

                        let filters = ctx.args.get(&context.entity_query_field.filters);
                        let filters = get_filter_conditions::<R>(context, filters)?;
                        let order_by = ctx.args.get(&context.entity_query_field.order_by);
                        let order_by = OrderInputBuilder { context }.parse_object::<R>(order_by)?;

                        let key = KeyComplex::<R> {
                            key: loader_impl::extract_key::<T::Model>(
                                &relation_definition.from_col,
                                parent,
                            )?,
                            meta: HashableGroupKey::<R> {
                                stmt,
                                junction_fields: Vec::new(),
                                rel_def: relation_definition,
                                via_def: None,
                                filters,
                                order_by,
                            },
                        };

                        let values = loader.load_one(key).await?;

                        let pagination = ctx.args.get(&context.entity_query_field.pagination);
                        let pagination =
                            PaginationInputBuilder { context }.parse_object(pagination)?;

                        let connection: Connection<R> =
                            apply_memory_pagination(context, values, pagination)?;

                        Ok(Some(FieldValue::owned_any(connection)))
                    })
                },
            ),
        };

        match relation_definition_is_owner {
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
