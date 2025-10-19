use crate::entities::*;
use async_graphql::dynamic::*;
use sea_orm::DatabaseConnection;
use seaography::{async_graphql, lazy_static::lazy_static, Builder, BuilderContext};

mod mutations;
mod queries;
mod types;

lazy_static! {
    static ref CONTEXT: BuilderContext = BuilderContext::default();
}

pub fn schema(
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    schema_builder(&CONTEXT, database, depth, complexity).finish()
}

pub fn schema_builder(
    context: &'static BuilderContext,
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> SchemaBuilder {
    let mut builder = Builder::new(context, database.clone());
    builder = register_entity_modules(builder);

    // // if `strict-custom-types` is enabled, add the following:
    // seaography::impl_custom_output_type_for_entities!([actor, ..]);

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

    seaography::register_complex_custom_outputs!(
        builder,
        [types::Rectangle, types::Circle, types::Triangle]
    );

    seaography::register_custom_unions!(builder, [types::Shape]);

    seaography::register_custom_queries!(builder, [queries::Operations]);

    seaography::register_custom_mutations!(builder, [mutations::Operations]);

    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .enable_uploading()
        .data(database)
}
