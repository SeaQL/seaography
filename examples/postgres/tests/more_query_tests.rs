use async_graphql::{dynamic::*, Response};
use sea_orm::{Database, DatabaseConnection};
use seaography::{async_graphql, lazy_static, Builder, BuilderContext, EntityQueryFieldConfig};
use seaography_postgres_example::entities::*;

lazy_static::lazy_static! {
    static ref CONTEXT : BuilderContext = {
        BuilderContext {
            entity_query_field: EntityQueryFieldConfig {
                use_ilike: true,
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
            inventory,
            language,
            payment,
            rental,
            staff,
            store,
        ]
    );
    builder.register_enumeration::<sea_orm_active_enums::MpaaRating>();
    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .data(database)
        .finish()
}

pub async fn get_schema() -> Schema {
    let database = Database::connect("postgres://sea:sea@127.0.0.1/sakila")
        .await
        .unwrap();
    let schema = schema(database, None, None).unwrap();

    schema
}

pub fn assert_eq(a: Response, b: &str) {
    assert_eq!(
        a.data.into_json().unwrap(),
        serde_json::from_str::<serde_json::Value>(b).unwrap()
    )
}

#[tokio::test]
async fn filter_ilike() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                  customer(filters: {
                    firstName: {
                      like: "MARY"
                    }
                  }) {
                    nodes {
                      customerId
                      firstName
                      lastName
                    }
                  }
                }
                "#,
            )
            .await,
        r#"
        {
          "customer": {
            "nodes": [
              {
                "customerId": 1,
                "firstName": "MARY",
                "lastName": "SMITH"
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
                  customer(filters: {
                    firstName: {
                      like: "mary"
                    }
                  }) {
                    nodes {
                      customerId
                      firstName
                      lastName
                    }
                  }
                }
                "#,
            )
            .await,
        r#"
        {
          "customer": {
            "nodes": [
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
                  customer(filters: {
                    firstName: {
                      ilike: "mario%"
                    }
                  }) {
                    nodes {
                      customerId
                      firstName
                      lastName
                    }
                  }
                }
                "#,
            )
            .await,
        r#"
        {
          "customer": {
            "nodes": [
              {
                "customerId": 178,
                "firstName": "MARION",
                "lastName": "SNYDER"
              },
              {
                "customerId": 441,
                "firstName": "MARIO",
                "lastName": "CHEATHAM"
              },
              {
                "customerId": 588,
                "firstName": "MARION",
                "lastName": "OCAMPO"
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
                  customer(filters: {
                    firstName: {
                      ilike: "mario"
                    }
                  }) {
                    nodes {
                      customerId
                      firstName
                      lastName
                    }
                  }
                }
                "#,
            )
            .await,
        r#"
        {
          "customer": {
            "nodes": [
              {
                "customerId": 441,
                "firstName": "MARIO",
                "lastName": "CHEATHAM"
              }
            ]
          }
        }
        "#,
    );
}
