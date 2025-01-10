use std::collections::BTreeMap;

use async_graphql::{dynamic::*, Response};
use sea_orm::{Database, DatabaseConnection};
use seaography::async_graphql;
use seaography::lazy_static;
use seaography::{Builder, BuilderContext, FnGuard, GuardsConfig};
use seaography_mysql_example::entities::*;

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

pub fn schema(
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    let mut builder = Builder::new(&CONTEXT, database.clone());
    seaography::register_entities!(
        builder,
        [
            actor,
            address,
            category,
            city,
            country,
            customer,
            film,
            film_actor,
            film_category,
            film_text,
            inventory,
            language,
            payment,
            rental,
            staff,
            store,
        ]
    );
    builder.register_enumeration::<sea_orm_active_enums::Rating>();
    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .data(database)
        .finish()
}

pub async fn get_schema() -> Schema {
    let database = Database::connect("mysql://sea:sea@127.0.0.1/sakila")
        .await
        .unwrap();
    let schema = schema(database, None, None).unwrap();

    schema
}

pub fn assert_eq(a: Response, b: &str) {
    assert_eq!(
        a.data.into_json().unwrap(),
        serde_json::from_str::<serde_json::Value>(b).unwrap()
    )
}

#[tokio::test]
async fn entity_guard_mutation() {
    let schema = get_schema().await;

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
    let schema = get_schema().await;

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
