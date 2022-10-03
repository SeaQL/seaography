use async_graphql::{dataloader::DataLoader, EmptyMutation, EmptySubscription, Response, Schema};
use sea_orm::Database;
use seaography_sqlite_example::{OrmDataloader, QueryRoot};

pub async fn get_schema() -> Schema<QueryRoot, EmptyMutation, EmptySubscription> {
    let database = Database::connect("sqlite://sakila.db").await.unwrap();
    let orm_dataloader: DataLoader<OrmDataloader> = DataLoader::new(
        OrmDataloader {
            db: database.clone(),
        },
        tokio::spawn,
    );
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(database)
        .data(orm_dataloader)
        .finish();

    schema
}

pub fn assert_eq(a: Response, b: &str) {
    assert_eq!(
        a.data.into_json().unwrap(),
        serde_json::from_str::<serde_json::Value>(b).unwrap()
    )
}

#[tokio::test]
async fn test_simple_query() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
          {
            store {
              data {
                storeId
                staff {
                  firstName
                  lastName
                }
              }
            }
          }
          "#,
            )
            .await,
        r#"
          {
            "store": {
              "data": [
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
          }
          "#,
    )
}

#[tokio::test]
async fn test_simple_query_with_filter() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
          {
              store(filters: {storeId:{eq: 1}}) {
                data {
                  storeId
                  staff {
                    firstName
                    lastName
                  }
                }
              }
          }
          "#,
            )
            .await,
        r#"
          {
            "store": {
              "data": [
                {
                  "storeId": 1,
                  "staff": {
                    "firstName": "Mike",
                    "lastName": "Hillyer"
                  }
                }
              ]
            }
          }
          "#,
    )
}

#[tokio::test]
async fn test_filter_with_pagination() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
            {
              customer (filters:{active:{eq: 0}}, pagination:{page: 2, limit: 3}) {
                data {
                  customerId
                }
                pages
                current
              }
            }
          "#,
            )
            .await,
        r#"
          {
            "customer": {
              "data": [
                {
                  "customerId": 315
                },
                {
                  "customerId": 368
                },
                {
                  "customerId": 406
                }
              ],
              "pages": 5,
              "current": 2
            }
          }
          "#,
    )
}

#[tokio::test]
async fn test_complex_filter_with_pagination() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
              {
                payment(filters:{amount: { gt: "11.1" }}, pagination: {limit: 2, page: 3}) {
                  data {
                    paymentId
                    amount
                  }
                  pages
                  current
                }
              }
          "#,
            )
            .await,
        r#"
          {
            "payment": {
              "data": [
                {
                  "paymentId": 8272,
                  "amount": "11.99"
                },
                {
                  "paymentId": 9803,
                  "amount": "11.99"
                }
              ],
              "pages": 5,
              "current": 3
            }
          }
          "#,
    )
}

#[tokio::test]
async fn test_cursor_pagination() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                  tracksCursor(cursor:{limit: 4, cursor: "Int[4]:2822"}, filters:{milliseconds: { gt: 2573031}}) {
                    edges {
                      node {
                        trackId
                        name
                        milliseconds
                      }
                      cursor
                    }
                  }
                }
        "#,
            )
            .await,
        r#"
        {
          "tracksCursor": {
            "edges": [
              {
                "node": {
                  "trackId": 2823,
                  "name": "Collaborators",
                  "milliseconds": 2626626
                },
                "cursor": "Int[4]:2823"
              },
              {
                "node": {
                  "trackId": 2824,
                  "name": "Torn",
                  "milliseconds": 2631291
                },
                "cursor": "Int[4]:2824"
              },
              {
                "node": {
                  "trackId": 2826,
                  "name": "Hero",
                  "milliseconds": 2713755
                },
                "cursor": "Int[4]:2826"
              },
              {
                "node": {
                  "trackId": 2827,
                  "name": "Unfinished Business",
                  "milliseconds": 2622038
                },
                "cursor": "Int[4]:2827"
              }
            ]
          }
        }
        "#,
    )
}
