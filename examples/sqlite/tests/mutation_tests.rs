use async_graphql::{dataloader::DataLoader, dynamic::*, Response};
use sea_orm::Database;
use seaography_sqlite_example::OrmDataloader;

pub async fn get_schema() -> Schema {
    let database = Database::connect("sqlite://sakila.db").await.unwrap();
    let orm_dataloader: DataLoader<OrmDataloader> = DataLoader::new(
        OrmDataloader {
            db: database.clone(),
        },
        tokio::spawn,
    );
    let schema =
        seaography_sqlite_example::query_root::schema(database, orm_dataloader, None, None)
            .unwrap();

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

#[tokio::test]
async fn test_create_batch_mutation() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                    filmText{
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
                    filmText{
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
