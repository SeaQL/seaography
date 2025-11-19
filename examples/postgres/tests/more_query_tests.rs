use async_graphql::{dynamic::*, Response};
use sea_orm::Database;
use seaography::{async_graphql, lazy_static, BuilderContext, EntityQueryFieldConfig};

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

async fn schema() -> Schema {
    let database = Database::connect("postgres://sea:sea@127.0.0.1/sakila")
        .await
        .unwrap();
    seaography_postgres_example::query_root::schema_builder(&CONTEXT, database, None, None)
        .finish()
        .unwrap()
}

fn assert_eq(a: Response, b: &str) {
    assert_eq!(
        a.data.into_json().unwrap(),
        serde_json::from_str::<serde_json::Value>(b).unwrap()
    )
}

#[tokio::test]
async fn filter_ilike() {
    let schema = schema().await;

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
