use async_graphql::{dynamic::*, Response};
use sea_orm::{Database, DatabaseConnection};
use seaography::{
    async_graphql, lazy_static, Builder, BuilderContext, DatabaseContext, EntityQueryFieldConfig,
};
use seaography_sqlite_example::entities::*;

lazy_static::lazy_static! {
    static ref CONTEXT : BuilderContext = {
        BuilderContext {
            entity_query_field: EntityQueryFieldConfig {
                combine_is_null_is_not_null: true,
                ..Default::default()
            },
            ..Default::default()
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
    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .data(database)
        .finish()
}

pub async fn get_schema() -> Schema {
    let database = Database::connect("sqlite://sakila.db").await.unwrap();
    let schema = schema(database.unrestricted(), None, None).unwrap();

    schema
}

pub fn assert_eq(a: Response, b: &str) {
    assert_eq!(
        a.data.into_json().unwrap(),
        serde_json::from_str::<serde_json::Value>(b).unwrap()
    )
}

#[tokio::test]
async fn filter_is_null() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                  address(
                    filters: { address: { contains: "Lane" } }
                    pagination: { page: { page: 0, limit: 2 } }
                  ) {
                    nodes {
                      addressId
                      address
                      address2
                      postalCode
                    }
                  }
                }
                "#,
            )
            .await,
        r#"
        {
          "address": {
            "nodes": [
              {
                "addressId": 3,
                "address": "23 Workhaven Lane",
                "address2": null,
                "postalCode": null
              },
              {
                "addressId": 19,
                "address": "419 Iligan Lane",
                "address2": null,
                "postalCode": "72878"
              }
            ]
          }
        }
        "#,
    );

    assert_eq(
        schema
            .execute(
                r#"
                {
                  address(
                    filters: { address: { contains: "Lane" }, postalCode: { is_null: false } }
                    pagination: { page: { page: 0, limit: 2 } }
                  ) {
                    nodes {
                      addressId
                      address
                      address2
                      postalCode
                    }
                  }
                }
                "#,
            )
            .await,
        r#"
        {
          "address": {
            "nodes": [
              {
                "addressId": 19,
                "address": "419 Iligan Lane",
                "address2": null,
                "postalCode": "72878"
              },
              {
                "addressId": 40,
                "address": "334 Munger (Monghyr) Lane",
                "address2": null,
                "postalCode": "38145"
              }
            ]
          }
        }
        "#,
    );

    assert_eq(
        schema
            .execute(
                r#"
                {
                  address(
                    filters: { address: { contains: "Lane" }, postalCode: { is_null: true } }
                    pagination: { page: { page: 0, limit: 2 } }
                  ) {
                    nodes {
                      addressId
                      address
                      address2
                      postalCode
                    }
                  }
                }
                "#,
            )
            .await,
        r#"
        {
          "address": {
            "nodes": [
              {
                "addressId": 3,
                "address": "23 Workhaven Lane",
                "address2": null,
                "postalCode": null
              }
            ]
          }
        }
        "#,
    );
}
