use std::collections::BTreeMap;

use async_graphql::{dynamic::*, Response};
use sea_orm::Database;
use seaography::{async_graphql, lazy_static, BuilderContext, FnGuard, GuardAction, GuardsConfig};

lazy_static::lazy_static! {
    static ref CONTEXT : BuilderContext = {
        let context = BuilderContext::default();
        let mut entity_guards: BTreeMap<String, FnGuard> = BTreeMap::new();
        entity_guards.insert("FilmCategory".into(), Box::new(|_ctx| {
            GuardAction::Block(None)
        }));
        let mut field_guards: BTreeMap<String, FnGuard> = BTreeMap::new();
        field_guards.insert("Language.lastUpdate".into(), Box::new(|_ctx| {
            GuardAction::Block(None)
        }));
        BuilderContext {
            guards: GuardsConfig {
                entity_guards,
                field_guards,
            },
            ..context
        }
    };
}

async fn schema() -> Schema {
    let database = Database::connect("sqlite://sakila.db").await.unwrap();
    seaography_sqlite_example::query_root::schema_builder(&CONTEXT, database, None, None)
        .finish()
        .unwrap()
}

fn assert_eq(a: Response, b: &str) {
    assert_eq!(
        a.data.into_json().unwrap(),
        serde_json::from_str::<serde_json::Value>(b).unwrap()
    )
}

#[tokio::test]
async fn entity_guard() {
    let schema = schema().await;

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
                  "name": "English"
                },
                {
                  "languageId": 2,
                  "name": "Italian"
                },
                {
                  "languageId": 3,
                  "name": "Japanese"
                },
                {
                  "languageId": 4,
                  "name": "Mandarin"
                },
                {
                  "languageId": 5,
                  "name": "French"
                },
                {
                  "languageId": 6,
                  "name": "German"
                }
              ]
            }
        }
        "#,
    );

    let response = schema
        .execute(
            r#"
        {
            filmCategory {
              nodes {
                filmId
              }
            }
        }
        "#,
        )
        .await;

    assert_eq!(response.errors.len(), 1);

    assert_eq!(response.errors[0].message, "Entity guard triggered.");
}

#[tokio::test]
async fn field_guard() {
    let schema = schema().await;

    let response = schema
        .execute(
            r#"
            {
                language {
                nodes {
                    languageId
                    name
                    lastUpdate
                }
                }
            }
        "#,
        )
        .await;

    assert_eq!(response.errors.len(), 1);

    assert_eq!(response.errors[0].message, "Field guard triggered.");
}
