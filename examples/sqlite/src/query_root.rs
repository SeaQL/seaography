use crate::entities::*;
use async_graphql::dynamic::*;
use sea_orm::DatabaseConnection;
use seaography::{async_graphql, lazy_static, Builder, BuilderContext};

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

    builder.mutations.extend([foo(), bar(), login()]);

    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .data(database)
        .finish()
}

use async_graphql::dynamic as gql_dyn;

fn foo() -> gql_dyn::Field {
    gql_dyn::Field::new(
        "foo",
        gql_dyn::TypeRef::named_nn(gql_dyn::TypeRef::STRING),
        move |ctx| {
            FieldFuture::new(async move {
                let username = ctx.args.try_get("username")?.string()?;

                let result = format!("Hello, {}!", username);
                Ok(Some(gql_dyn::FieldValue::value(result)))
            })
        },
    )
    .argument(gql_dyn::InputValue::new(
        "username",
        gql_dyn::TypeRef::named_nn(gql_dyn::TypeRef::STRING),
    ))
}

fn bar() -> gql_dyn::Field {
    gql_dyn::Field::new(
        "bar",
        gql_dyn::TypeRef::named_nn(gql_dyn::TypeRef::INT),
        move |ctx| {
            FieldFuture::new(async move {
                let x = ctx.args.try_get("x")?.i64()?;
                let y = ctx.args.try_get("y")?.i64()?;

                let result = x + y;
                Ok(Some(gql_dyn::FieldValue::value(result)))
            })
        },
    )
    .argument(gql_dyn::InputValue::new(
        "x",
        gql_dyn::TypeRef::named_nn(gql_dyn::TypeRef::INT),
    ))
    .argument(gql_dyn::InputValue::new(
        "y",
        gql_dyn::TypeRef::named_nn(gql_dyn::TypeRef::INT),
    ))
}

fn login() -> gql_dyn::Field {
    gql_dyn::Field::new(
        "login",
        gql_dyn::TypeRef::named_nn("Customer"),
        move |ctx| {
            FieldFuture::new(async move {
                use sea_orm::EntityTrait;

                let repo = ctx.data::<DatabaseConnection>().unwrap();

                let result = customer::Entity::find().one(repo).await?.unwrap();
                Ok(Some(gql_dyn::FieldValue::owned_any(result)))
            })
        },
    )
}
