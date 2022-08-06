use async_graphql::dataloader::DataLoader;
use async_graphql::{EmptyMutation, EmptySubscription, Response, Schema};
use generated::{OrmDataloader, QueryRoot};
use sea_orm::Database;

pub async fn get_schema() -> Schema<QueryRoot, EmptyMutation, EmptySubscription> {
    let database = Database::connect("mysql://sea:sea@127.0.0.1/sakila")
        .await
        .unwrap();
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
                  lastUpdate
                  addressAddress {
                    address
                  }
                  storeStaff {
                    firstName
                    lastName
                  }
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
            "store": {
              "data": [
                {
                  "storeId": 1,
                  "lastUpdate": "2006-02-15T04:57:12+00:00",
                  "addressAddress": {
                    "address": "47 MySakila Drive"
                  },
                  "storeStaff": [
                    {
                      "firstName": "Mike",
                      "lastName": "Hillyer"
                    }
                  ]
                },
                {
                  "storeId": 2,
                  "lastUpdate": "2006-02-15T04:57:12+00:00",
                  "addressAddress": {
                    "address": "28 MySQL Boulevard"
                  },
                  "storeStaff": [
                    {
                      "firstName": "Jon",
                      "lastName": "Stephens"
                    }
                  ]
                }
              ],
              "pages": 1,
              "current": 1
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
                      lastUpdate
                      addressAddress {
                        address
                      }
                      storeStaff {
                        firstName
                        lastName
                      }
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
          "store": {
              "data": [
                {
                  "storeId": 1,
                  "lastUpdate": "2006-02-15T04:57:12+00:00",
                  "addressAddress": {
                    "address": "47 MySakila Drive"
                  },
                  "storeStaff": [
                    {
                      "firstName": "Mike",
                      "lastName": "Hillyer"
                    }
                  ]
                }
              ],
              "pages": 1,
              "current": 1
          }
        }
        "#,
    )
}