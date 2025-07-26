use async_graphql::{dynamic::*, Response};
use sea_orm::Database;
use seaography::async_graphql;

pub async fn get_schema() -> Schema {
    let database = Database::connect("sqlite://sakila.db").await.unwrap();
    let schema = seaography_sqlite_example::query_root::schema(database, None, None).unwrap();

    schema
}

pub fn assert_eq(a: Response, b: &str) {
    assert_eq!(
        a.data.into_json().unwrap(),
        serde_json::from_str::<serde_json::Value>(b).unwrap()
    )
}

#[tokio::test]
async fn test_custom_query_with_pagination() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                  customer_of_store2(pagination: { page: { page: 0, limit: 2 } }) {
                    nodes {
                      storeId
                      customerId
                      lastName
                      email
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
          "customer_of_store2": {
            "nodes": [
              {
                "storeId": 2,
                "customerId": 4,
                "lastName": "JONES",
                "email": "BARBARA.JONES@sakilacustomer.org"
              },
              {
                "storeId": 2,
                "customerId": 6,
                "lastName": "DAVIS",
                "email": "JENNIFER.DAVIS@sakilacustomer.org"
              }
            ],
            "paginationInfo": {
              "pages": 137,
              "current": 0
            }
          }
        }
        "#,
    );

    assert_eq(
        schema
            .execute(
                r#"
                {
                  customer_of_store2(pagination: { page: { page: 1, limit: 1 } }) {
                    nodes {
                      storeId
                      customerId
                      lastName
                      email
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
          "customer_of_store2": {
            "nodes": [
              {
                "storeId": 2,
                "customerId": 6,
                "lastName": "DAVIS",
                "email": "JENNIFER.DAVIS@sakilacustomer.org"
              }
            ],
            "paginationInfo": {
              "pages": 273,
              "current": 1
            }
          }
        }
        "#,
    );
}
