use async_graphql::{dynamic::*, Response};
use sea_orm::Database;
use seaography::async_graphql;

async fn schema() -> Schema {
    let database = Database::connect("sqlite://sakila.db").await.unwrap();
    seaography_sqlite_example::query_root::schema(database, None, None).unwrap()
}

fn assert_eq(a: Response, b: &str) {
    assert_eq!(
        a.data.into_json().unwrap(),
        serde_json::from_str::<serde_json::Value>(b).unwrap()
    )
}

#[tokio::test]
async fn test_custom_mutations() {
    let schema = schema().await;

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
async fn test_custom_mutation_with_custom_input() {
    let schema = schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                mutation {
                  rental_request(rental_request: {
                    customer: "Alice"
                    film: "Star Wars"
                    timestamp: "2025-01-01 02:03:05 UTC"
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

    assert_eq(
        schema
            .execute(
                r#"
                mutation {
                  rental_request(rental_request: {
                    customer: "Alice"
                    film: "Star Wars"
                    location: {
                      city: "Riverside"
                    }
                    timestamp: "2025-01-01 02:03:05 UTC"
                  })
                }
                "#,
            )
            .await,
        r#"
        {
          "rental_request": "Alice wants to rent Star Wars (at Riverside)"
        }
        "#,
    );

    assert_eq(
        schema
            .execute(
                r#"
                mutation {
                  rental_request(rental_request: {
                    customer: "Alice"
                    film: "Star Wars"
                    location: {
                      city: "Riverside"
                      county: "West"
                    }
                    timestamp: "2025-01-01 02:03:05 UTC"
                  })
                }
                "#,
            )
            .await,
        r#"
        {
          "rental_request": "Alice wants to rent Star Wars (at Riverside, West)"
        }
        "#,
    );
}

#[tokio::test]
async fn test_custom_mutation_with_optional_custom_input() {
    let schema = schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                mutation {
                  maybe_rental_request {
                    rentalId
                  }
                }
                "#,
            )
            .await,
        r#"
        {
          "maybe_rental_request": null
        }
        "#,
    );

    assert_eq(
        schema
            .execute(
                r#"
                mutation {
                  maybe_rental_request(rental_request: {
                    customer: "Bob"
                    film: "Star Trek"
                    timestamp: "2022-11-14 10:30:10 UTC"
                  }) {
                    rentalId
                    lastUpdate
                  }
                }
                "#,
            )
            .await,
        r#"
        {
          "maybe_rental_request": {
            "rentalId": 1,
            "lastUpdate": "2022-11-14 10:30:12 UTC"
          }
        }
        "#,
    );
}

#[tokio::test]
async fn test_custom_mutation_with_vec_custom_input() {
    let schema = schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                mutation {
                  many_rental_request(rental_requests: [
                    {
                      customer: "Alice"
                      film: "Star Wars"
                      timestamp: "2022-11-14T10:30:10+00:00"
                    },
                    {
                      customer: "Bob"
                      film: "Star Trek"
                      timestamp: "2022-11-14 10:30:10 UTC"
                    }
                  ])
                }
                "#,
            )
            .await,
        r#"
        {
          "many_rental_request": 2
        }
        "#,
    );
}
