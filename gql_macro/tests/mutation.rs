use async_graphql::Context;
use sea_orm::{DatabaseConnection, DeriveEntityModel, entity::prelude::*};
use seaography::{Builder, BuilderContext, EntityObjectBuilder, lazy_static, impl_gql};
use gql_macro::mutation;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl_gql!(Model where context: &CONTEXT);

pub struct TestMutation;

#[mutation]
impl TestMutation {
    async fn foo(&self, _ctx: &Context<'_>, username: String) -> async_graphql::Result<String> {
        Ok(format!("Hello, {}!", username))
    }

    async fn bar(&self, _ctx: &Context<'_>, x: i32, y: i32) -> async_graphql::Result<i32> {
        Ok(x + y)
    }

    async fn baz(&self, _ctx: &Context<'_>) -> async_graphql::Result<Model> {
        Ok(Model {
            id: 1,
            name: "John".to_string(),
            email: "john@example.com".to_string(),
        })
    }
}

lazy_static::lazy_static! { static ref CONTEXT: BuilderContext = BuilderContext::default (); }

#[tokio::test]
async fn test_mutations() {
    let mut builder = Builder::new(&CONTEXT, DatabaseConnection::Disconnected);

    let entity_object_builder = EntityObjectBuilder { context: &CONTEXT };

    builder
        .outputs
        .push(entity_object_builder.to_object::<Entity>());

    builder.mutations.extend(TestMutation.into_dynamic_fields());

    let schema_builder = builder.schema_builder();

    let schema = schema_builder.finish().unwrap();

    let query = r#"
    mutation {
        baz {
            id
            name
            email
        }
    }
    "#;

    let res = schema.execute(query).await;

    println!("{:#?}", res);
}