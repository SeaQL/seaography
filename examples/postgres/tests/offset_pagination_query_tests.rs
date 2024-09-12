use async_graphql::{dynamic::*, Response};
use sea_orm::Database;

pub async fn get_schema() -> Schema {
    let database = Database::connect("postgres://sea:sea@127.0.0.1/sakila")
        .await
        .unwrap();
    let schema = seaography_postgres_example::query_root::schema(database, None, None).unwrap();

    schema
}

pub fn assert_eq(a: Response, b: &str) {
    assert_eq!(
        a.data.into_json().unwrap(),
        serde_json::from_str::<serde_json::Value>(b).unwrap()
    )
}

#[cfg(feature = "offset-pagination")]
#[tokio::test]
async fn test_simple_query() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                    store {
                        storeId
                        staff {
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
            "store": [
            {
                "storeId": 1,
                "staff": {
                    "firstName": "Mike",
                    "lastName": "Hillyer"
                }
            },
            {
                "storeId": 2,
                "staff": {
                    "firstName": "Jon",
                    "lastName": "Stephens"
                }
            }
            ]
        }
          "#,
    )
}

#[cfg(feature = "offset-pagination")]
#[tokio::test]
async fn test_simple_query_with_filter() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
              {
                  store(filters: {storeId:{eq: 1}}) {
                      storeId
                      staff {
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
                    "store": [
                        {
                        "storeId": 1,
                        "staff": {
                            "firstName": "Mike",
                            "lastName": "Hillyer"
                        }
                    }
                    ]
                }
                "#,
    )
}

#[cfg(feature = "offset-pagination")]
#[tokio::test]
async fn test_filter_with_pagination() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
              {
                  customer(
                    filters: { active: { eq: 0 } }
                    pagination: { page: { page: 2, limit: 3 } }
                  ) {
                     customerId
                  }
                }
              "#,
            )
            .await,
        r#"
                {
                    "customer": [
                        {
                          "customerId": 315
                        },
                        {
                          "customerId": 368
                        },
                        {
                          "customerId": 406
                        }
                    ]
                }
                "#,
    )
}