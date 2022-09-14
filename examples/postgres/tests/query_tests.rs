use async_graphql::{dataloader::DataLoader, EmptyMutation, EmptySubscription, Response, Schema};
use sea_orm::Database;
use seaography_postgres_example::{OrmDataloader, QueryRoot};

pub async fn get_schema() -> Schema<QueryRoot, EmptyMutation, EmptySubscription> {
    let database = Database::connect("postgres://sea:sea@127.0.0.1/sakila")
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
                    "amount": "11.9900"
                  },
                  {
                    "paymentId": 9803,
                    "amount": "11.9900"
                  }
                ],
                "pages": 5,
                "current": 3
              }
            }
            "#,
    )
}
