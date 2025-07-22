use super::*;
use async_graphql::Result as GqlResult;
use sea_orm::DbErr;
use seaography::macros::CustomOperation;

/*

mutation {
  foo(username: "hi")
  bar(x: 2, y: 3)
  login {
    customerId
    firstName
    lastName
  }
}

*/
pub fn def() -> Vec<Field> {
    [Foo::gql(), Bar::gql(), Login::gql()]
        .into_iter()
        .flatten()
        .collect()
}

#[allow(dead_code)]
#[derive(CustomOperation)]
struct Foo {
    foo: fn(username: String) -> String,
    foo_hi: fn(username: String) -> String,
}

#[allow(dead_code)]
#[derive(CustomOperation)]
struct Bar {
    bar: fn(x: i32, y: i32) -> i32,
    bar_sub: fn(x: i32, y: i32) -> i32,
}

#[allow(dead_code)]
#[derive(CustomOperation)]
struct Login {
    login: fn() -> customer::Model,
    logout: fn(username: String) -> String,
}

impl Foo {
    async fn foo(_ctx: &ResolverContext<'_>, username: String) -> GqlResult<String> {
        Ok(format!("Hello, {}!", username))
    }

    async fn foo_hi(_ctx: &ResolverContext<'_>, username: String) -> GqlResult<String> {
        Ok(format!("Hi, {}!", username))
    }
}

impl Bar {
    async fn bar(_ctx: &ResolverContext<'_>, x: i32, y: i32) -> GqlResult<i32> {
        Ok(x + y)
    }

    async fn bar_sub(_ctx: &ResolverContext<'_>, x: i32, y: i32) -> GqlResult<i32> {
        Ok(x - y)
    }
}

impl Login {
    async fn login(ctx: &ResolverContext<'_>) -> GqlResult<customer::Model> {
        use sea_orm::EntityTrait;

        let repo = ctx.data::<DatabaseConnection>().unwrap();
        Ok(customer::Entity::find()
            .one(repo)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound("Customer not found".to_owned()))?)
    }

    async fn logout(_ctx: &ResolverContext<'_>, username: String) -> GqlResult<String> {
        Ok(format!("Bye, {}!", username))
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
