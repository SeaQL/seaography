use crate::entities::*;
use async_graphql::dynamic::*;
use sea_orm::DatabaseConnection;
use seaography::{Builder, BuilderContext, ActiveEnumConfig, heck::ToSnakeCase};

lazy_static::lazy_static! {
    static ref CONTEXT: BuilderContext = BuilderContext {
        active_enum: ActiveEnumConfig {
            variant_name: Box::new(|_enum_name: &str, variant: &str| -> String {
                variant.to_snake_case()
            }),
            ..Default::default()
        },
        ..Default::default()
    };
}

pub fn schema(
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    let mut builder = Builder::new(&CONTEXT, database.clone());
    seaography::register_entities!(
        builder,
        [
            film_actor,
            rental,
            category,
            staff,
            country,
            film,
            actor,
            language,
            city,
            inventory,
            film_category,
            customer,
            store,
            payment,
            address,
        ]
    );
    builder.register_enumeration::<crate::entities::sea_orm_active_enums::MpaaRating>();
    let schema = builder.schema_builder();
    let schema = if let Some(depth) = depth {
        schema.limit_depth(depth)
    } else {
        schema
    };
    let schema = if let Some(complexity) = complexity {
        schema.limit_complexity(complexity)
    } else {
        schema
    };
    schema.data(database).finish()
}
