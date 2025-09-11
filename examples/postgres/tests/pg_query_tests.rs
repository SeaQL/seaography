use async_graphql::{dynamic::*, Response};
use sea_orm::Database;
use seaography::async_graphql;

pub async fn get_schema() -> Schema {
    let database = Database::connect(
        &std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://sea:sea@127.0.0.1/sakila".to_owned()),
    )
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
async fn test_film_query() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                  film(filters: {
                    title: { contains: "LIFE" }
                  }) {
                    nodes {
                      filmId
                      title
                      specialFeatures
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
                  "filmId": 25,
                  "title": "ANGELS LIFE",
                  "specialFeatures": [
                    "Trailers"
                  ]
                },
                {
                  "filmId": 522,
                  "title": "LIFE TWISTED",
                  "specialFeatures": [
                    "Commentaries",
                    "Deleted Scenes"
                  ]
                }
              ]
            }
        }
          "#,
    )
}

#[tokio::test]
async fn test_film_query_by_array_contains() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                  film(filters: {
                    title: { contains: "LIFE" }
                    specialFeatures: { array_contains: ["Trailers"] }
                  }) {
                    nodes {
                      filmId
                      title
                      specialFeatures
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
                  "filmId": 25,
                  "title": "ANGELS LIFE",
                  "specialFeatures": [
                    "Trailers"
                  ]
                }
              ]
            }
        }
          "#,
    )
}

#[tokio::test]
async fn test_film_query_by_array_overlap() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                  film(filters: {
                    title: { contains: "LIFE" }
                    specialFeatures: { array_overlap: ["Commentaries", "Documentary"] }
                  }) {
                    nodes {
                      filmId
                      title
                      specialFeatures
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
                  "filmId": 522,
                  "title": "LIFE TWISTED",
                  "specialFeatures": [
                    "Commentaries",
                    "Deleted Scenes"
                  ]
                }
              ]
            }
        }
          "#,
    )
}

#[tokio::test]
async fn test_film_query_by_json_eq() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                  film(
                    filters: {
                      filmId: { is_in: [1, 2, 3] }
                    }
                    orderBy: { filmId: ASC }
                  ) {
                    nodes {
                      filmId
                      title
                      metadata
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
                "filmId": 1,
                "title": "ACADEMY DINOSAUR",
                "metadata": {
                  "bar": "baz",
                  "foo": 123
                }
              },
              {
                "filmId": 2,
                "title": "ACE GOLDFINGER",
                "metadata": {
                  "bar": "boo",
                  "foo": 456
                }
              },
              {
                "filmId": 3,
                "title": "ADAPTATION HOLES",
                "metadata": null
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
                {
                  film(
                    filters: {
                      filmId: { is_in: [1, 2, 3] }
                      metadata: { eq: { bar: "baz", foo: 123 } }
                    }
                  ) {
                    nodes {
                      filmId
                      title
                      metadata
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
                "filmId": 1,
                "title": "ACADEMY DINOSAUR",
                "metadata": {
                  "bar": "baz",
                  "foo": 123
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
                {
                  film(
                    filters: {
                      filmId: { is_in: [1, 2, 3] }
                      metadata: { ne: { bar: "baz", foo: 123 } }
                    }
                  ) {
                    nodes {
                      filmId
                      title
                      metadata
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
                "filmId": 2,
                "title": "ACE GOLDFINGER",
                "metadata": {
                  "bar": "boo",
                  "foo": 456
                }
              }
            ]
          }
        }
        "#,
    );
}
