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
                  storeStoreStaff {
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
                    "storeStoreStaff": [
                      {
                        "firstName": "Mike",
                        "lastName": "Hillyer"
                      }
                    ]
                  },
                  {
                    "storeId": 2,
                    "storeStoreStaff": [
                      {
                        "firstName": "Jon",
                        "lastName": "Stephens"
                      }
                    ]
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
                    storeStoreStaff {
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
                      "storeStoreStaff": [
                        {
                          "firstName": "Mike",
                          "lastName": "Hillyer"
                        }
                      ]
                    }
                  ]
                }
            }
            "#,
    )
}
