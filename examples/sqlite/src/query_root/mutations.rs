use super::*;
use async_graphql::{Result as GqlResult, Upload};
use custom_inputs::RentalRequest;
use sea_orm::{DbErr, EntityTrait};
use seaography::macros::CustomOperation;

/*

mutation {
  foo(username: "hi")
  bar(x: 2, y: 3)
  upload(upload: File)
  login {
    customerId
    firstName
    lastName
  }
}

*/
#[allow(dead_code)]
#[derive(CustomOperation)]
pub struct Operations {
    foo: fn(username: String) -> String,
    bar: fn(x: i32, y: i32) -> i32,
    login: fn() -> customer::Model,
    rental_request: fn(rental_request: RentalRequest) -> String,
    upload: fn(upload: Upload) -> String,
    #[rustfmt::skip]
    maybe_rental_request: fn(rental_request: Option::<RentalRequest>) -> Option::<rental::Model>,
}

impl Operations {
    async fn upload(ctx: &ResolverContext<'_>, upload: Upload) -> GqlResult<String> {
        Ok(format!(
            "upload: filename={}",
            upload.value(ctx).unwrap().filename
        ))
    }

    async fn foo(_ctx: &ResolverContext<'_>, username: String) -> GqlResult<String> {
        Ok(format!("Hello, {}!", username))
    }

    async fn bar(_ctx: &ResolverContext<'_>, x: i32, y: i32) -> GqlResult<i32> {
        Ok(x + y)
    }

    async fn login(ctx: &ResolverContext<'_>) -> GqlResult<customer::Model> {
        use sea_orm::EntityTrait;

        let db = ctx.data::<DatabaseConnection>().unwrap();
        Ok(customer::Entity::find()
            .one(db)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound("Customer not found".to_owned()))?)
    }

    async fn rental_request(
        _ctx: &ResolverContext<'_>,
        rental_request: RentalRequest,
    ) -> GqlResult<String> {
        let mut s = format!(
            "{} wants to rent {}",
            rental_request.customer, rental_request.film
        );
        if let Some(location) = rental_request.location {
            use std::fmt::Write;
            write!(&mut s, " (at {}", location.city).unwrap();
            if let Some(county) = location.county {
                write!(&mut s, ", {}", county).unwrap();
            }
            write!(&mut s, ")").unwrap();
        }
        Ok(s)
    }

    async fn maybe_rental_request(
        ctx: &ResolverContext<'_>,
        rental_request: Option<RentalRequest>,
    ) -> GqlResult<Option<rental::Model>> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        Ok(match rental_request {
            Some(_) => rental::Entity::find().one(db).await?,
            None => None,
        })
    }
}

/*
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
*/
/*
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
*/
/*
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
*/
