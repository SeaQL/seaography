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
}

async fn schema() -> Schema {
    let database = Database::connect("postgres://sea:sea@127.0.0.1/sakila")
        .await
        .unwrap();
    seaography_postgres_example::query_root::schema(database, None, None).unwrap()
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
              filmActorDelete(filter: { lastUpdate: { gt: "2022-11-14 10:30:12" } })
            }
            "#,
        )
        .await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                    filmActor(filters: { lastUpdate: { gt: "2022-11-14 10:30:12" } }) {
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
                    filmActorCreateOne(data: { actorId: 1, filmId: 2,  lastUpdate: "2030-01-01 11:11:11"}) {
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
                    filmActor(filters: { lastUpdate: { gt: "2022-11-14 10:30:12" } }) {
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
                  rentalDate: { eq: "2030-01-25 21:50:05" }
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
        .execute(
            r#"
                mutation {
                  rentalCreateOne(
                    data: {
                      rentalDate: "2030-01-25 21:50:05"
                      inventoryId: 4452
                      customerId: 319
                      returnDate: "2030-01-12 21:50:05"
                      staffId: 1
                      lastUpdate: "2030-01-01 21:50:05"
                    }
                  ) {
                    rentalId
                    inventoryId
                    customerId
                    returnDate
                    staffId
                  }
                }
                "#,
        )
        .await
        .data
        .into_json()
        .unwrap();

    let result: CreateResult = serde_json::from_value(response).unwrap();
    let rental = result.rental_create_one;
    let rental_id = rental.rental_id;
    assert!(rental.rental_id > max_id);
    assert_eq!(
        rental,
        RentalObject {
            rental_id,
            inventory_id: 4452,
            customer_id: 319,
            staff_id: 1,
            return_date: "2030-01-12 21:50:05".into(),
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
                "returnDate": "2030-01-12 21:50:05"
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
              languageDelete(filter: { languageId: { gt: 6 } })
            }
            "#,
        )
        .await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                  language(orderBy: { languageId: ASC }) {
                    nodes {
                      languageId
                      name
                    }
                  }
                }
                "#,
            )
            .await,
        r#"
            {
              "language": {
                "nodes": [
                  {
                    "languageId": 1,
                    "name": "English             "
                  },
                  {
                    "languageId": 2,
                    "name": "Italian             "
                  },
                  {
                    "languageId": 3,
                    "name": "Japanese            "
                  },
                  {
                    "languageId": 4,
                    "name": "Mandarin            "
                  },
                  {
                    "languageId": 5,
                    "name": "French              "
                  },
                  {
                    "languageId": 6,
                    "name": "German              "
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
                  languageCreateBatch(
                    data: [
                      { name: "Swedish", lastUpdate: "2030-01-12 21:50:05" }
                      { name: "Danish", lastUpdate: "2030-01-12 21:50:05" }
                    ]
                  ) {
                    name
                  }
                }
                "#,
            )
            .await,
        r#"
            {
              "languageCreateBatch": [
                {
                  "name": "Swedish             "
                },
                {
                  "name": "Danish              "
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
                  language(orderBy: { languageId: ASC }) {
                    nodes {
                      name
                    }
                  }
                }
                "#,
            )
            .await,
        r#"

        {
            "language": {
                "nodes": [
                    {
                        "name": "English             "
                    },
                    {
                        "name": "Italian             "
                    },
                    {
                        "name": "Japanese            "
                    },
                    {
                        "name": "Mandarin            "
                    },
                    {
                        "name": "French              "
                    },
                    {
                        "name": "German              "
                    },
                    {
                        "name": "Swedish             "
                    },
                    {
                        "name": "Danish              "
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
              languageDelete(filter: { name: { is_in: ["Swedish", "Danish"] } })
            }
            "#,
        )
        .await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                  language(filters: { languageId: { gt: 6 } }) {
                    nodes {
                      languageId
                      name
                    }
                  }
                }
                "#,
            )
            .await,
        r#"
            {
              "language": {
                "nodes": [
                ]
              }
            }
            "#,
    );
}

async fn test_update_mutation() {
    let schema = schema().await;

    restore_country(&schema).await;

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

    restore_country(&schema).await;
}

async fn restore_country(schema: &Schema) {
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

    assert_eq(
        schema
            .execute(
                r#"
                {
                  language(filters: { languageId: { gte: 9 } }, orderBy: { languageId: ASC }) {
                    nodes {
                      languageId
                    }
                  }
                }
                "#,
            )
            .await,
        r#"
        {
            "language": {
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
                  languageCreateBatch(
                    data: [
                      { name: "9", lastUpdate: "2030-01-12 21:50:05" }
                      { name: "10", lastUpdate: "2030-01-12 21:50:05" }
                      { name: "11", lastUpdate: "2030-01-12 21:50:05" }
                    ]
                  ) {
                    name
                  }
                }
                "#,
            )
            .await,
        r#"
        {
            "languageCreateBatch": [
              {
                "name": "9                   "
              },
              {
                "name": "10                  "
              },
              {
                "name": "11                  "
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
                  language(filters: { languageId: { gte: 9 } }, orderBy: { languageId: ASC }) {
                    nodes {
                      name
                    }
                  }
                }
                "#,
            )
            .await,
        r#"
        {
            "language": {
              "nodes": [
                {
                  "name": "9                   "
                },
                {
                  "name": "10                  "
                },
                {
                  "name": "11                  "
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
                  languageDelete(filter: { languageId: { gt: 6 } })
                }
                "#,
            )
            .await,
        r#"
            {
              "languageDelete": 3
            }
            "#,
    );

    assert_eq(
        schema
            .execute(
                r#"
                {
                  language(filters: { languageId: { gt: 6 } }, orderBy: { languageId: ASC }) {
                    nodes {
                      languageId
                    }
                  }
                }
                "#,
            )
            .await,
        r#"
        {
          "language": {
            "nodes": [
            ]
          }
        }
        "#,
    );
}
