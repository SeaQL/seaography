use async_graphql::{dynamic::*, Response};
use sea_orm::Database;
use seaography::{
    async_graphql, lazy_static, BuilderContext, EntityQueryFieldConfig, PaginationInputConfig,
    TypesMapConfig,
};

lazy_static::lazy_static! {
    static ref CONTEXT : BuilderContext = {
        BuilderContext {
            entity_query_field: EntityQueryFieldConfig {
                combine_is_null_is_not_null: true,
                ..Default::default()
            },
            pagination_input: PaginationInputConfig{
              default_limit: Some(3),
              max_limit: Some(10),
              ..Default::default()
            },
            types: TypesMapConfig {
              timestamp_rfc3339: true,
              ..Default::default()
            },
            ..Default::default()
        }
    };
}

async fn schema() -> Schema {
    let database = Database::connect("sqlite://sakila.db").await.unwrap();
    seaography_sqlite_example::query_root::schema_builder(&CONTEXT, database, None, None)
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
async fn filter_is_null() {
    let schema = schema().await;

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

#[tokio::test]
async fn test_default_pagination() {
    let schema = schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                  customer {
                    nodes {
                      customerId
                      firstName
                      lastName
                    }
                    paginationInfo {
                      pages
                      current
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
              },
              {
                "customerId": 2,
                "firstName": "PATRICIA",
                "lastName": "JOHNSON"
              },
              {
                "customerId": 3,
                "firstName": "LINDA",
                "lastName": "WILLIAMS"
              }
            ],
            "paginationInfo": {
              "pages": 200,
              "current": 0
            }
          }
        }
        "#,
    );
}

#[tokio::test]
async fn test_pagination_error() {
    let schema = schema().await;

    let response = schema
        .execute(
            r#"
            {
              customer(
                pagination: { page: { page: 1, limit: 11 } }
              ) {
                nodes {
                  customerId
                }
                paginationInfo {
                  pages
                  current
                }
              }
            }
            "#,
        )
        .await;

    assert_eq!(response.errors.len(), 1);

    assert_eq!(
        response.errors[0].message,
        "Query Error: Requested pagination limit (11) exceeds maximum allowed (10)"
    );
}

#[tokio::test]
async fn test_timestamp_format() {
    let schema = schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                    film(filters:{filmId: {eq: 1}}) {
                      nodes {
                        title
                        lastUpdate
                      }
                    }
                }
                "#,
            )
            .await,
        r#"
        {
            "film": {
              "nodes": [
                {
                  "title": "ACADEMY DINOSAUR",
                  "lastUpdate": "2022-11-14T10:30:09+00:00"
                }
              ]
            }
        }
        "#,
    )
}
