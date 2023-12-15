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

#[tokio::test]
async fn test_simple_insert_one() {
    let schema = get_schema().await;

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

#[tokio::test]
async fn test_complex_insert_one() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                    rental(filters: { rentalId: { eq: 16050 } }) {
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
            .await,
        r#"
        {
            "rental": {
              "nodes": [
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
                    rentalCreateOne(
                      data: {
                        rentalId: 16050
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
            .await,
        r#"
            {
                "rentalCreateOne": {
                    "rentalId": 16050,
                    "inventoryId": 4452,
                    "customerId": 319,
                    "returnDate": "2030-01-12 21:50:05",
                    "staffId": 1
                }
            }
            "#,
    );

    assert_eq(
        schema
            .execute(
                r#"
                {
                    rental(filters: { rentalId: { eq: 16050 } }) {
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
            .await,
        r#"
        {
            "rental": {
              "nodes": [
                {
                  "rentalId": 16050,
                  "inventoryId": 4452,
                  "customerId": 319,
                  "returnDate": "2030-01-12 21:50:05",
                  "staffId": 1
                }
              ]
            }
        }
        "#,
    );
}

#[tokio::test]
async fn test_create_batch_mutation() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                    language {
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
                        { languageId: 7, name: "Swedish", lastUpdate: "2030-01-12 21:50:05" }
                        { languageId: 8, name: "Danish", lastUpdate: "2030-01-12 21:50:05" }
                      ]
                    ) {
                      languageId
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
                    "languageId": 7,
                    "name": "Swedish             "
                },
                {
                    "languageId": 8,
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
                    language {
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
                    },
                    {
                        "languageId": 7,
                        "name": "Swedish             "
                    },
                    {
                        "languageId": 8,
                        "name": "Danish              "
                    }
                ]
            }
        }
        "#,
    )
}

#[tokio::test]
async fn test_update_mutation() {
    let schema = get_schema().await;

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
}
