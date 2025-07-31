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
async fn test_custom_mutations() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                mutation {
                  foo(username: "hi")
                  bar(x: 2, y: 3)
                  login {
                    customerId
                    firstName
                    lastName
                  }
                }
                "#,
            )
            .await,
        r#"
        {
          "foo": "Hello, hi!",
          "bar": 5,
          "login": {
            "customerId": 1,
            "firstName": "MARY",
            "lastName": "SMITH"
          }
        }
        "#,
    );
}

#[tokio::test]
async fn test_custom_mutation_with_custom_entities() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                mutation {
                  rental_request(rental_request: {
                    customer: "Alice"
                    film: "Star Wars"
                  })
                }
                "#,
            )
            .await,
        r#"
        {
          "rental_request": "Alice wants to rent Star Wars"
        }
        "#,
    );
}
