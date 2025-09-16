use crate::entities::*;
use async_graphql::dynamic::*;
use sea_orm::DatabaseConnection;
use seaography::{async_graphql, lazy_static, Builder, BuilderContext};

mod mutations;
mod queries;
mod types;

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
            actor,
            address,
            category,
            city,
            country,
            customer,
            film,
            film_actor,
            film_category,
            film_text,
            inventory,
            language,
            payment,
            rental,
            staff,
            store,
        ]
    );

    seaography::register_custom_inputs!(
        builder,
        [
            types::RentalRequest,
            types::Location,
            types::Point,
            types::Size,
            types::Rectangle,
            types::Circle,
            types::Triangle,
            types::Shape,
        ]
    );

    seaography::register_custom_outputs!(
        builder,
        [
            types::PurchaseOrder,
            types::Lineitem,
            types::ProductSize,
            types::Point,
            types::Size,
        ]
    );

    builder.register_custom_output_with_fields::<types::Rectangle>();
    builder.register_custom_output_with_fields::<types::Circle>();
    builder.register_custom_output_with_fields::<types::Triangle>();

    builder.register_custom_union::<types::Shape>();

    seaography::register_custom_queries!(builder, [queries::Operations]);

    seaography::register_custom_mutations!(builder, [mutations::Operations]);

    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .enable_uploading()
        .data(database)
        .finish()
}
