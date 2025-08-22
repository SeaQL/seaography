use crate::entities::*;
use async_graphql::dynamic::*;
use sea_orm::DatabaseConnection;
use seaography::{async_graphql, lazy_static, Builder, BuilderContext};

mod custom_inputs;
mod mutations;
mod queries;

lazy_static::lazy_static! { static ref CONTEXT : BuilderContext = BuilderContext :: default () ; }

pub fn schema(
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    let engine: sea_orm::RbacEngineHolder = Default::default();
    engine.replace(sea_orm::rbac::RbacEngine::from_snapshot(sea_orm::rbac::RbacSnapshot::danger_unrestricted()));
    let database =  sea_orm::RestrictedConnection {
        user_id: sea_orm::rbac::RbacUserId(0),
        conn: database,
        engine,
    };

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
        [custom_inputs::RentalRequest, custom_inputs::Location]
    );

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
