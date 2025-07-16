use crate::entities::*;
use async_graphql::dynamic::*;
use async_graphql::Context;
use sea_orm::{DatabaseConnection, EntityTrait};
use seaography::{async_graphql, lazy_static, Builder, BuilderContext, impl_gql};
use gql_macro::mutation;

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

    builder
        .mutations
        .extend((CustomMutations {}).into_dynamic_fields());

    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .data(database)
        .finish()
}

pub struct CustomMutations {}

impl_gql!(customer::Model);

#[mutation]
impl CustomMutations {
    async fn foo(&self, _ctx: &Context<'_>, username: String) -> async_graphql::Result<String> {
        Ok(format!("Hello, {}!", username))
    }

    async fn bar(&self, _ctx: &Context<'_>, x: i32, y: i32) -> async_graphql::Result<i32> {
        Ok(x + y)
    }

    async fn login(&self, ctx: &Context<'_>) -> async_graphql::Result<customer::Model> {
        let repo = ctx.data::<DatabaseConnection>().unwrap();
        Ok(customer::Entity::find().one(repo).await?.unwrap())
    }
}