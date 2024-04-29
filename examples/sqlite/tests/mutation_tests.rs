use async_graphql::{dynamic::*, Response};
use sea_orm::Database;

#[tokio::test]
async fn main() {
    test_simple_insert_one().await;
    test_complex_insert_one().await;
    test_create_batch_mutation().await;
    test_update_mutation().await;
    test_delete_mutation().await;
}

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

async fn test_simple_insert_one() {
    let schema = get_schema().await;

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
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                    rental(filters: { rentalId: { eq: 16050 } }) {
                      nodes {
                        rentalId
                        rentalDate
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
                        rentalDate: "2030-01-01 11:11:11 UTC"
                        inventoryId: 4452
                        customerId: 319
                        returnDate: "2030-01-01 11:11:11 UTC"
                        staffId: 1
                        lastUpdate: "2030-01-01 11:11:11 UTC"
                      }
                    ) {
                      rentalId
                      rentalDate
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
                    "rentalDate": "2030-01-01 11:11:11 UTC",
                    "inventoryId": 4452,
                    "customerId": 319,
                    "returnDate": "2030-01-01 11:11:11 UTC",
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
                        rentalDate
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
                  "rentalDate": "2030-01-01 11:11:11 UTC",
                  "inventoryId": 4452,
                  "customerId": 319,
                  "returnDate": "2030-01-01 11:11:11 UTC",
                  "staffId": 1
                }
              ]
            }
        }
            "#,
    );
}

async fn test_create_batch_mutation() {
    let schema = get_schema().await;

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
}

async fn test_update_mutation() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                    country(pagination: { page: { limit: 10, page: 0 } }) {
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
                },
                {
                  "country": "Armenia",
                  "countryId": 7
                },
                {
                  "country": "Australia",
                  "countryId": 8
                },
                {
                  "country": "Austria",
                  "countryId": 9
                },
                {
                  "country": "Azerbaijan",
                  "countryId": 10
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
                    country(pagination: { page: { limit: 10, page: 0 } }) {
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
                },
                {
                  "country": "Armenia",
                  "countryId": 7
                },
                {
                  "country": "Australia",
                  "countryId": 8
                },
                {
                  "country": "Austria",
                  "countryId": 9
                },
                {
                  "country": "Azerbaijan",
                  "countryId": 10
                }
              ]
            }
        }
        "#,
    );
}

async fn test_delete_mutation() {
    let schema = get_schema().await;

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
        r#"
            {
                "filmTextDelete": 2
            }
            "#,
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
}
