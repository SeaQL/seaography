use std::collections::BTreeMap;

use crate::entities::*;
use async_graphql::dynamic::*;
use sea_orm::DatabaseConnection;
use seaography::{async_graphql, lazy_static, Builder, BuilderContext, FnGuard, GuardsConfig};

mod mutations;
mod queries;

lazy_static::lazy_static! {
    static ref CONTEXT : BuilderContext = {
        let context = BuilderContext::default();
        let mut entity_guards: BTreeMap<String, FnGuard> = BTreeMap::new();
        entity_guards.insert("FilmCategory".into(), Box::new(|_ctx| {
            seaography::GuardAction::Block(None)
        }));
        entity_guards.insert("Language:create".into(), Box::new(|_ctx| {
            seaography::GuardAction::Block(None)
        }));
        entity_guards.insert("Language:update".into(), Box::new(|_ctx| {
            seaography::GuardAction::Block(None)
        }));
        entity_guards.insert("Language:delete".into(), Box::new(|_ctx| {
            seaography::GuardAction::Block(None)
        }));
        let mut field_guards: BTreeMap<String, FnGuard> = BTreeMap::new();
        field_guards.insert("Language.lastUpdate".into(), Box::new(|_ctx| {
            seaography::GuardAction::Block(None)
        }));
        BuilderContext {
            guards: GuardsConfig {
                entity_guards,
                field_guards,
            },
            ..context
        }
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

    builder.queries.extend(queries::Operations::to_fields());

    builder.mutations.extend(mutations::Operations::to_fields());

    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .data(database)
        .finish()
}