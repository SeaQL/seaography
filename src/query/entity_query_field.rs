use async_graphql::dynamic::{Field, FieldFuture, FieldValue, InputValue, TypeRef};
use heck::{ToLowerCamelCase, ToSnakeCase};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    apply_guard, apply_order, apply_pagination, get_filter_conditions, guard_error,
    pluralize_unique, BuilderContext, ConnectionObjectBuilder, DatabaseContext,
    EntityObjectBuilder, FilterInputBuilder, GuardAction, OperationType, OrderInputBuilder,
    PaginationInputBuilder, UserContext,
};

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
    /// if false, `is_null` and `is_not_null` are two separate fields.
    /// if true, `is_null` accepts `bool` and `is_null: false` means `is_not_null`;
    /// `is_not_null` will be removed.
    pub combine_is_null_is_not_null: bool,
}

impl std::default::Default for EntityQueryFieldConfig {
    fn default() -> Self {
        EntityQueryFieldConfig {
            type_name: Box::new(|object_name: &str| -> String {
                if cfg!(feature = "field-snake-case") {
                    object_name.to_snake_case()
                } else {
                    object_name.to_lower_camel_case()
                }
            }),
            filters: "filters".into(),
            order_by: {
                if cfg!(feature = "field-snake-case") {
                    "order_by"
                } else {
                    "orderBy"
                }
                .into()
            },
            pagination: "pagination".into(),
            combine_is_null_is_not_null: false,
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
    {
        let entity_object = EntityObjectBuilder {
            context: self.context,
        };
        let object_name = pluralize_unique(&entity_object.type_name::<T>(), false);
        self.context.entity_query_field.type_name.as_ref()(&object_name)
    }

    fn type_name_vanilla<T>(&self) -> String
    where
        T: EntityTrait,
    {
        let entity_object = EntityObjectBuilder {
            context: self.context,
        };
        let object_name = &entity_object.type_name::<T>();
        self.context.entity_query_field.type_name.as_ref()(object_name)
    }

    /// the additional singular Query endpoint
    pub fn to_singular_field<T>(&self) -> Field
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        use sea_orm::{ColumnTrait, Iterable, PrimaryKeyToColumn};

        use crate::TypesMapHelper;

        let entity_object = EntityObjectBuilder {
            context: self.context,
        };

        let object_name = entity_object.type_name::<T>();

        let guard = self.context.guards.entity_guards.get(&object_name);
        let hooks = &self.context.hooks;

        let column = T::PrimaryKey::iter()
            .map(|variant| variant.into_column())
            .collect::<Vec<T::Column>>()[0];

        let column_def = column.def();
        let enum_type_name = column.enum_type_name();
        let types_helper = TypesMapHelper {
            context: self.context,
        };

        let converted_type = types_helper.sea_orm_column_type_to_graphql_type(
            column_def.get_column_type(),
            true,
            enum_type_name,
        );

        let iv = InputValue::new("id", converted_type.expect("primary key to be supported"));

        Field::new(
            self.type_name::<T>(),
            TypeRef::named_nn(&object_name),
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

                    #[allow(unused_mut)]
                    let mut stmt = T::find();
                    #[cfg(feature = "with-uuid")]
                    {
                        let mapper = ctx.data::<crate::TypesMapHelper>()?;
                        let column = T::PrimaryKey::iter()
                            .map(|variant| variant.into_column())
                            .collect::<Vec<T::Column>>()[0];

                        let v = mapper.async_graphql_value_to_sea_orm_value::<T>(
                            &column,
                            &ctx.args.try_get("id")?,
                        )?;
                        stmt = stmt.filter(column.eq(v));
                    }

                    let db = &ctx
                        .data::<DatabaseConnection>()?
                        .restricted(ctx.data_opt::<UserContext>())?;

                    let r = stmt.one(db).await?;

                    Ok(Some(FieldValue::owned_any(r)))
                })
            },
        )
        .argument(iv)
    }

    /// used to get the Query object field for a SeaORM entity
    pub fn to_field<T>(&self) -> Field
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
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

        let object_name = pluralize_unique(&entity_object.type_name::<T>(), true);
        let object_name_ = object_name.clone();
        let type_name = connection_object_builder.type_name(&object_name);

        let guard = self.context.guards.entity_guards.get(&object_name);
        let hooks = &self.context.hooks;
        let context: &'static BuilderContext = self.context;
        let connection_name = pluralize_unique(&self.type_name_vanilla::<T>(), true);

        Field::new(connection_name, TypeRef::named_nn(type_name), move |ctx| {
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

                let filters = ctx.args.get(&context.entity_query_field.filters);
                let filters = get_filter_conditions::<T>(context, filters)?;
                let order_by = ctx.args.get(&context.entity_query_field.order_by);
                let order_by = OrderInputBuilder { context }.parse_object::<T>(order_by)?;
                let pagination = ctx.args.get(&context.entity_query_field.pagination);
                let pagination = PaginationInputBuilder { context }.parse_object(pagination)?;

                let mut stmt = T::find();
                if let Some(filter) = hooks.entity_filter(&ctx, &object_name, OperationType::Read) {
                    stmt = stmt.filter(filter);
                }
                stmt = stmt.filter(filters);
                stmt = apply_order(stmt, order_by);

                let db = &ctx
                    .data::<DatabaseConnection>()?
                    .restricted(ctx.data_opt::<UserContext>())?;

                let connection = apply_pagination::<T>(db, stmt, pagination).await?;

                Ok(Some(FieldValue::owned_any(connection)))
            })
        })
        .argument(InputValue::new(
            &self.context.entity_query_field.filters,
            TypeRef::named(filter_input_builder.type_name(&object_name_)),
        ))
        .argument(InputValue::new(
            &self.context.entity_query_field.order_by,
            TypeRef::named(order_input_builder.type_name(&object_name_)),
        ))
        .argument(InputValue::new(
            &self.context.entity_query_field.pagination,
            TypeRef::named(pagination_input_builder.type_name()),
        ))
    }
}
