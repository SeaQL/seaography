use crate::entities::*;
use async_graphql::dynamic::*;
use sea_orm::DatabaseConnection;
use seaography::async_graphql;
use seaography::lazy_static;
use seaography::{Builder, BuilderContext};

lazy_static::lazy_static! { static ref CONTEXT : BuilderContext = BuilderContext :: default () ; }

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
    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .data(database)
        .finish()
}
