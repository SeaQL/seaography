use crate::entities::*;
use async_graphql::dynamic::*;
use sea_orm::DatabaseConnection;
use seaography::{async_graphql, lazy_static, Builder, BuilderContext};

mod mutations;

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

    builder.queries.push(custom_query());

    builder.mutations.extend(mutations::Endpoints::to_fields());

    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .data(database)
        .finish()
}

fn custom_query() -> Field {
    Field::new(
        "custom_query",
        TypeRef::named_nn("CustomerConnection"),
        move |ctx| {
            FieldFuture::new(async move {
                use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
                use seaography::{apply_pagination, PaginationInputBuilder};

                let db = ctx.data::<DatabaseConnection>()?;

                let stmt = customer::Entity::find().filter(customer::Column::StoreId.eq(2));
                let pagination = ctx.args.get("pagination");
                let pagination =
                    PaginationInputBuilder { context: &CONTEXT }.parse_object(pagination);
                let connection = apply_pagination::<customer::Entity>(db, stmt, pagination).await?;

                Ok(Some(FieldValue::owned_any(connection)))
            })
        },
    )
    .argument(InputValue::new(
        "pagination",
        TypeRef::named("PaginationInput"),
    ))
}
