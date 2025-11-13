use async_graphql::{dynamic::*, Response};
use sea_orm::Database;
use seaography::async_graphql;
use serde::Deserialize;

#[tokio::test]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();

    test_simple_insert_one().await;
    test_complex_insert_one().await;
    test_create_batch_mutation().await;
    test_update_mutation().await;
    test_delete_mutation().await;
    test_add_original_language_to_film().await;
}

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

async fn test_simple_insert_one() {
    let schema = schema().await;

    schema
        .execute(
            r#"
            mutation {
              filmActorDelete(filter: { lastUpdate: { gt: "2022-11-14 10:30:12 UTC" } })
            }
            "#,
        )
        .await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                    filmActor(filters: { lastUpdate: { gt: "2022-11-14 10:30:12 UTC" } }) {
                      nodes {
                        actorId
                        filmId
                      }
                    }
                }
                "#,
            )
            .await,
        r#"
        {
            "filmActor": {
              "nodes": []
            }
          }
            "#,
    );

    assert_eq(
        schema
            .execute(
                r#"
                mutation {
                    filmActorCreateOne(data: { actorId: 1, filmId: 2,  lastUpdate: "2030-01-01 11:11:11 UTC"}) {
                      actorId
                      filmId
                         __typename
                    }
                }
                "#,
            )
            .await,
        r#"
            {
                "filmActorCreateOne": {
                    "actorId": 1,
                    "filmId": 2,
                    "__typename": "FilmActorBasic"
                }
            }
            "#,
    );

    assert_eq(
        schema
            .execute(
                r#"
                {
                    filmActor(filters: { lastUpdate: { gt: "2022-11-14 10:30:12 UTC" } }) {
                      nodes {
                        actorId
                        filmId
                      }
                    }
                }
                "#,
            )
            .await,
        r#"
                {
                    "filmActor": {
                        "nodes": [
                            {
                                "actorId": 1,
                                "filmId": 2
                            }
                        ]
                    }
                }
            "#,
    );
}

async fn test_complex_insert_one() {
    let schema = schema().await;

    schema
        .execute(
            r#"
            mutation {
              rentalDelete(
                filter: {
                  rentalDate: { eq: "2030-01-25 21:50:05 UTC" }
                  inventoryId: { eq: 4452 }
                  customerId: { eq: 319 }
                }
              )
            }
            "#,
        )
        .await;

    #[derive(Deserialize)]
    struct QueryResult {
        rental: RentalQuery,
    }

    #[derive(Deserialize)]
    struct RentalQuery {
        nodes: Vec<RentalId>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct RentalId {
        rental_id: i32,
    }

    let response = schema
        .execute(
            r#"
                {
                    rental(orderBy: { rentalId: DESC }) {
                      nodes {
                        rentalId
                        inventoryId
                        customerId
                        returnDate
                        staffId
                      }
                    }
                }
                "#,
        )
        .await
        .data
        .into_json()
        .unwrap();

    let result: QueryResult = serde_json::from_value(response).unwrap();
    let max_id = result
        .rental
        .nodes
        .iter()
        .map(|r| r.rental_id)
        .max()
        .unwrap_or_default();
    let rental_id = max_id + 1;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct CreateResult {
        rental_create_one: RentalObject,
    }

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "camelCase")]
    struct RentalObject {
        rental_id: i32,
        inventory_id: i32,
        customer_id: i32,
        staff_id: i32,
        return_date: String,
    }

    let response = schema
        .execute(format!(
            r#"
            mutation {{
              rentalCreateOne(
                data: {{
                  rentalId: {rental_id}
                  rentalDate: "2030-01-25 21:50:05 UTC"
                  inventoryId: 4452
                  customerId: 319
                  returnDate: "2030-01-12 21:50:05 UTC"
                  staffId: 1
                  lastUpdate: "2030-01-01 21:50:05 UTC"
                }}
              ) {{
                rentalId
                inventoryId
                customerId
                returnDate
                staffId
              }}
            }}
            "#,
        ))
        .await
        .data
        .into_json()
        .unwrap();

    let result: CreateResult = serde_json::from_value(response).unwrap();
    let rental = result.rental_create_one;
    assert_eq!(rental.rental_id, rental_id);
    assert_eq!(
        rental,
        RentalObject {
            rental_id,
            inventory_id: 4452,
            customer_id: 319,
            staff_id: 1,
            return_date: "2030-01-12 21:50:05 UTC".into(),
        }
    );

    assert_eq(
        schema
            .execute(format!(
                r#"
                {{
                  rental(filters: {{ rentalId: {{ eq: {rental_id} }} }}) {{
                    nodes {{
                      rentalId
                      inventoryId
                      customerId
                      staffId
                      returnDate
                    }}
                  }}
                }}
                "#
            ))
            .await,
        &format!(
            r#"
        {{
          "rental": {{
            "nodes": [
              {{
                "rentalId": {rental_id},
                "inventoryId": 4452,
                "customerId": 319,
                "staffId": 1,
                "returnDate": "2030-01-12 21:50:05 UTC"
              }}
            ]
          }}
        }}
        "#
        ),
    );
}

async fn test_create_batch_mutation() {
    let schema = schema().await;

    schema
        .execute(
            r#"
            mutation {
              filmTextDelete(filter: { })
            }
            "#,
        )
        .await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                    filmText(filters: { filmId: { lte: 5 } }, orderBy: { filmId: ASC }) {
                      nodes {
                        filmId
                        title
                        description
                      }
                    }
                }
                "#,
            )
            .await,
        r#"
            {
                "filmText": {
                "nodes": []
                }
            }
            "#,
    );

    assert_eq(
        schema
            .execute(
                r#"
                mutation {
                    filmTextCreateBatch(
                      data: [
                        { filmId: 1, title: "TEST 1", description: "TEST DESC 1" }
                        { filmId: 2, title: "TEST 2", description: "TEST DESC 2" }
                      ]
                    ) {
                      filmId
                      title
                      description
                    }
                }
                "#,
            )
            .await,
        r#"
            {
                "filmTextCreateBatch": [
                {
                    "filmId": 1,
                    "title": "TEST 1",
                    "description": "TEST DESC 1"
                },
                {
                    "filmId": 2,
                    "title": "TEST 2",
                    "description": "TEST DESC 2"
                }
                ]
            }
            "#,
    );

    assert_eq(
        schema
            .execute(
                r#"
                {
                    filmText(filters: { filmId: { lte: 5 } }, orderBy: { filmId: ASC }) {
                      nodes {
                        filmId
                        title
                        description
                      }
                    }
                }
                "#,
            )
            .await,
        r#"
            {
                "filmText": {
                    "nodes": [
                        {
                            "filmId": 1,
                            "title": "TEST 1",
                            "description": "TEST DESC 1"
                        },
                        {
                            "filmId": 2,
                            "title": "TEST 2",
                            "description": "TEST DESC 2"
                        }
                    ]
                }
            }
            "#,
    );

    schema
        .execute(
            r#"
            mutation {
              filmTextDelete(filter: { })
            }
            "#,
        )
        .await;
}

async fn test_update_mutation() {
    let schema = schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                  country(filters: { countryId: { lt: 7 } }, orderBy: { countryId: ASC }) {
                    nodes {
                      country
                      countryId
                    }
                  }
                }
                "#,
            )
            .await,
        r#"
        {
            "country": {
              "nodes": [
                {
                  "country": "Afghanistan",
                  "countryId": 1
                },
                {
                  "country": "Algeria",
                  "countryId": 2
                },
                {
                  "country": "American Samoa",
                  "countryId": 3
                },
                {
                  "country": "Angola",
                  "countryId": 4
                },
                {
                  "country": "Anguilla",
                  "countryId": 5
                },
                {
                  "country": "Argentina",
                  "countryId": 6
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
                mutation {
                  countryUpdate(
                    data: { country: "[DELETED]" }
                    filter: { countryId: { lt: 6 } }
                  ) {
                    countryId
                    country
                  }
                }
                "#,
            )
            .await,
        r#"
        {
            "countryUpdate": [
              {
                "countryId": 1,
                "country": "[DELETED]"
              },
              {
                "countryId": 2,
                "country": "[DELETED]"
              },
              {
                "countryId": 3,
                "country": "[DELETED]"
              },
              {
                "countryId": 4,
                "country": "[DELETED]"
              },
              {
                "countryId": 5,
                "country": "[DELETED]"
              }
            ]
        }
        "#,
    );

    assert_eq(
        schema
            .execute(
                r#"
                {
                  country(filters: { countryId: { lt: 7 } }, orderBy: { countryId: ASC }) {
                    nodes {
                      country
                      countryId
                    }
                  }
                }
                "#,
            )
            .await,
        r#"
        {
            "country": {
              "nodes": [
                {
                  "country": "[DELETED]",
                  "countryId": 1
                },
                {
                  "country": "[DELETED]",
                  "countryId": 2
                },
                {
                  "country": "[DELETED]",
                  "countryId": 3
                },
                {
                  "country": "[DELETED]",
                  "countryId": 4
                },
                {
                  "country": "[DELETED]",
                  "countryId": 5
                },
                {
                  "country": "Argentina",
                  "countryId": 6
                }
              ]
            }
        }
        "#,
    );

    schema
        .execute(
            r#"mutation {
              countryUpdate(data: { country: "Afghanistan" } filter: { countryId: { eq: 1 } }) { country }
            }"#,
        )
        .await;
    schema
        .execute(
            r#"mutation {
              countryUpdate(data: { country: "Algeria" } filter: { countryId: { eq: 2 } }) { country }
            }"#,
        )
        .await;
    schema
        .execute(
            r#"mutation {
              countryUpdate(data: { country: "American Samoa" } filter: { countryId: { eq: 3 } }) { country }
            }"#,
        )
        .await;
    schema
        .execute(
            r#"mutation {
              countryUpdate(data: { country: "Angola" } filter: { countryId: { eq: 4 } }) { country }
            }"#,
        )
        .await;
    schema
        .execute(
            r#"mutation {
              countryUpdate(data: { country: "Anguilla" } filter: { countryId: { eq: 5 } }) { country }
            }"#,
        )
        .await;
}

async fn test_delete_mutation() {
    let schema = schema().await;

    schema
        .execute(
            r#"
        mutation {
            filmTextCreateBatch(
              data: [
                { filmId: 6, title: "TEST 6", description: "TEST DESC 6" }
                { filmId: 7, title: "TEST 7", description: "TEST DESC 7" }
                { filmId: 8, title: "TEST 8", description: "TEST DESC 8" }
              ]
            ) {
              filmId
            }
        }
        "#,
        )
        .await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                    filmText(filters: { filmId: { gte: 6 } }, orderBy: { filmId: ASC }) {
                      nodes {
                        filmId
                        title
                      }
                    }
                }
                "#,
            )
            .await,
        r#"
        {
            "filmText": {
              "nodes": [
                {
                  "filmId": 6,
                  "title": "TEST 6"
                },
                {
                  "filmId": 7,
                  "title": "TEST 7"
                },
                {
                  "filmId": 8,
                  "title": "TEST 8"
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
                mutation {
                  filmTextDelete(filter: { filmId: { gte: 7 } })
                }
                "#,
            )
            .await,
        r#"{ "filmTextDelete": 2 }"#,
    );

    assert_eq(
        schema
            .execute(
                r#"
                {
                    filmText(filters: { filmId: { gte: 6 } }, orderBy: { filmId: ASC }) {
                      nodes {
                        filmId
                        title
                      }
                    }
                }
                "#,
            )
            .await,
        r#"
        {
            "filmText": {
              "nodes": [
                {
                  "filmId": 6,
                  "title": "TEST 6"
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
                mutation {
                  filmTextDelete(filter: { filmId: { eq: 6 } })
                }
                "#,
            )
            .await,
        r#"{ "filmTextDelete": 1 }"#,
    );
}

async fn test_add_original_language_to_film() {
    let schema = schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                  film(filters: { filmId: { eq: 500 } }) {
                    nodes {
                      filmId
                      title
                      language1 {
                        name
                      }
                      language2 {
                        name
                      }
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
                "filmId": 500,
                "title": "KISS GLORY",
                "language1": {
                  "name": "English"
                },
                "language2": null
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
                mutation {
                    filmUpdate(
                      data: { originalLanguageId: 5 }
                      filter: { filmId: { eq: 500 } }
                    ) {
                      filmId
                      title
                    }
                }
                "#,
            )
            .await,
        r#"
        {
            "filmUpdate": [
              {
                "filmId": 500,
                "title": "KISS GLORY"
              }
            ]
        }
        "#,
    );

    assert_eq(
        schema
            .execute(
                r#"
                {
                  film(filters: { filmId: { eq: 500 } }) {
                    nodes {
                      filmId
                      title
                      language1 {
                        name
                      }
                      language2 {
                        name
                      }
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
                "filmId": 500,
                "title": "KISS GLORY",
                "language1": {
                  "name": "English"
                },
                "language2": {
                  "name": "French"
                }
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
                mutation {
                    filmUpdate(
                      data: { originalLanguageId: null }
                      filter: { filmId: { eq: 500 } }
                    ) {
                      filmId
                      title
                    }
                }
                "#,
            )
            .await,
        r#"
        {
            "filmUpdate": [
              {
                "filmId": 500,
                "title": "KISS GLORY"
              }
            ]
        }
        "#,
    );

    assert_eq(
        schema
            .execute(
                r#"
                {
                  film(filters: { filmId: { eq: 500 } }) {
                    nodes {
                      filmId
                      title
                      language1 {
                        name
                      }
                      language2 {
                        name
                      }
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
                "filmId": 500,
                "title": "KISS GLORY",
                "language1": {
                  "name": "English"
                },
                "language2": null
              }
            ]
          }
        }
        "#,
    );
}
