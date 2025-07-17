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
        <String as seaography::AsyncGqlValueType>::gql_type_ref(&CONTEXT),
        move |ctx| {
            FieldFuture::new(async move {
                let username = <String as seaography::AsyncGqlValueType>::try_get_arg(
                    &CONTEXT, &ctx, "username",
                )?;

                let result = format!("Hello, {}!", username);
                Ok(Some(gql_dyn::FieldValue::value(result)))
            })
        },
    )
    .argument(gql_dyn::InputValue::new(
        "username",
        <String as seaography::AsyncGqlValueType>::gql_type_ref(&CONTEXT),
    ))
}

fn bar() -> gql_dyn::Field {
    gql_dyn::Field::new(
        "bar",
        <i32 as seaography::AsyncGqlValueType>::gql_type_ref(&CONTEXT),
        move |ctx| {
            FieldFuture::new(async move {
                let x = <i32 as seaography::AsyncGqlValueType>::try_get_arg(&CONTEXT, &ctx, "x")?;
                let y = <i32 as seaography::AsyncGqlValueType>::try_get_arg(&CONTEXT, &ctx, "y")?;

                let result = x + y;
                Ok(Some(gql_dyn::FieldValue::value(result)))
            })
        },
    )
    .argument(gql_dyn::InputValue::new(
        "x",
        <i32 as seaography::AsyncGqlValueType>::gql_type_ref(&CONTEXT),
    ))
    .argument(gql_dyn::InputValue::new(
        "y",
        <i32 as seaography::AsyncGqlValueType>::gql_type_ref(&CONTEXT),
    ))
}

fn login() -> gql_dyn::Field {
    gql_dyn::Field::new(
        "login",
        <seaography::SeaOrmModel<customer::Model> as seaography::AsyncGqlValueType>::gql_type_ref(
            &CONTEXT,
        ),
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
