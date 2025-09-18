use std::collections::BTreeMap;

use async_graphql::{dynamic::*, Response};
use sea_orm::{Database, DatabaseConnection};
use seaography::{async_graphql, lazy_static, Builder, BuilderContext, FnGuard, GuardsConfig};

lazy_static::lazy_static! {
    static ref CONTEXT : BuilderContext = {
        let context = BuilderContext::default();
        let mut entity_guards: BTreeMap<String, FnGuard> = BTreeMap::new();
        entity_guards.insert("FilmCategory".into(), Box::new(|_ctx| {
            seaography::GuardAction::Block(None)
        }));
        let mut field_guards: BTreeMap<String, FnGuard> = BTreeMap::new();
        field_guards.insert("Language.name".into(), Box::new(|_ctx| {
            seaography::GuardAction::Block(None)
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
async fn entity_guard_mutation() {
    let schema = schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                mutation LanguageUpdate {
                    languageUpdate(
                        data: { lastUpdate: "2030-01-01 11:11:11 UTC" }
                        filter: { languageId: { eq: 6 } }
                    ) {
                        languageId
                    }
                }
                "#,
            )
            .await,
        r#"
        {
            "languageUpdate": [
                {
                    "languageId": 6
                }
            ]
        }
        "#,
    );

    let response = schema
        .execute(
            r#"
            mutation FilmCategoryUpdate {
                filmCategoryUpdate(
                    data: { filmId: 1, categoryId: 1, lastUpdate: "2030-01-01 11:11:11 UTC" }
                ) {
                    filmId
                }
            }
"#,
        )
        .await;

    assert_eq!(response.errors.len(), 1);

    assert_eq!(response.errors[0].message, "Entity guard triggered.");

    let response = schema
        .execute(
            r#"
        mutation FilmCategoryDelete {
            filmCategoryDelete(filter: { filmId: { eq: 2 } })
        }
"#,
        )
        .await;

    assert_eq!(response.errors.len(), 1);

    assert_eq!(response.errors[0].message, "Entity guard triggered.");
}

#[tokio::test]
async fn field_guard_mutation() {
    let schema = schema().await;

    let response = schema
        .execute(
            r#"
            mutation LanguageUpdate {
                languageUpdate(data: { name: "Cantonese" }, filter: { languageId: { eq: 6 } }) {
                    languageId
                }
            }      
    "#,
        )
        .await;

    assert_eq!(response.errors.len(), 1);

    assert_eq!(response.errors[0].message, "Field guard triggered.");
}
