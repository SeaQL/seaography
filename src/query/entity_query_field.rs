use async_graphql::{
    dynamic::{Field, FieldFuture, FieldValue, InputValue, TypeRef},
    Error,
};
use heck::ToLowerCamelCase;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    apply_order, apply_pagination, get_filter_conditions, BuilderContext, ConnectionObjectBuilder,
    EntityObjectBuilder, FilterInputBuilder, OrderInputBuilder, PaginationInputBuilder,
};

pub struct EntityQueryFieldConfig {
    pub type_name: Box<dyn Fn(&str) -> String + Sync + Send>,
    pub filters: String,
    pub order_by: String,
    pub pagination: String,
}

impl std::default::Default for EntityQueryFieldConfig {
    fn default() -> Self {
        EntityQueryFieldConfig {
            type_name: Box::new(|object_name: &str| -> String {
                object_name.to_lower_camel_case()
            }),
            filters: "filters".into(),
            order_by: "orderBy".into(),
            pagination: "pagination".into(),
        }
    }
}

pub struct EntityQueryFieldBuilder {
    pub context: &'static BuilderContext,
}

impl EntityQueryFieldBuilder {
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

        let object_name = entity_object.type_name::<T>();
        let type_name = connection_object_builder.type_name(&object_name);

        let guard = self.context.guards.entity_guards.get(&object_name);

        let context: &'static BuilderContext = self.context;
        Field::new(
            entity_object.query_entity_name::<T>(),
            TypeRef::named_nn(type_name),
            move |ctx| {
                let context: &'static BuilderContext = context;
                FieldFuture::new(async move {
                    let guard_flag = if let Some(guard) = guard {
                        (*guard)(&ctx)
                    } else {
                        false
                    };

                    if guard_flag {
                        return Err(Error::new("Entity guard triggered."));
                    }

                    let filters = ctx.args.get(&context.entity_query_field.filters);
                    let order_by = ctx.args.get(&context.entity_query_field.order_by);
                    let pagination = ctx.args.get(&context.entity_query_field.pagination);

                    let stmt = T::find();
                    let stmt = stmt.filter(get_filter_conditions::<T>(context, filters));
                    let stmt = apply_order(context, stmt, order_by);

                    let db = ctx.data::<DatabaseConnection>()?;

                    let connection = apply_pagination::<T>(context, db, stmt, pagination).await?;

                    Ok(Some(FieldValue::owned_any(connection)))
                })
            },
        )
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
