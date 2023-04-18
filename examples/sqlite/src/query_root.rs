use crate::{entities::*, OrmDataloader};
use async_graphql::{dataloader::DataLoader, dynamic::*};
use sea_orm::DatabaseConnection;
use seaography::{Builder, BuilderContext};

lazy_static::lazy_static! { static ref CONTEXT : BuilderContext = BuilderContext :: default () ; }

pub fn schema(
    database: DatabaseConnection,
    orm_dataloader: DataLoader<OrmDataloader>,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    let mut builder = Builder::new(&CONTEXT);

    seaography::register_entity!(builder, &CONTEXT, film_actor);
    seaography::register_entity!(builder, &CONTEXT, rental);
    seaography::register_entity!(builder, &CONTEXT, category);
    seaography::register_entity!(builder, &CONTEXT, staff);
    seaography::register_entity!(builder, &CONTEXT, country);
    seaography::register_entity!(builder, &CONTEXT, film);
    seaography::register_entity!(builder, &CONTEXT, actor);
    seaography::register_entity!(builder, &CONTEXT, language);
    seaography::register_entity!(builder, &CONTEXT, city);
    seaography::register_entity!(builder, &CONTEXT, inventory);
    seaography::register_entity!(builder, &CONTEXT, film_text);
    seaography::register_entity!(builder, &CONTEXT, film_category);
    seaography::register_entity!(builder, &CONTEXT, customer);
    seaography::register_entity!(builder, &CONTEXT, store);
    seaography::register_entity!(builder, &CONTEXT, payment);
    seaography::register_entity!(builder, &CONTEXT, address);

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
    schema.data(database).data(orm_dataloader).finish()
}
