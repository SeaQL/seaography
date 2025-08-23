use async_graphql::{dynamic::*, Response};
use sea_orm::{Database, DatabaseConnection};
use seaography::{
    async_graphql, lazy_static, Builder, BuilderContext, DatabaseContext, GuardAction,
    LifecycleHooks, LifecycleHooksInterface, OperationType,
};
use seaography_sqlite_example::entities::*;

lazy_static::lazy_static! {
    static ref CONTEXT : BuilderContext = {
        BuilderContext {
            hooks: LifecycleHooks::new(MyHooks),
            ..Default::default()
        }
    };
}

struct MyHooks;

impl LifecycleHooksInterface for MyHooks {
    fn entity_guard(
        &self,
        _ctx: &ResolverContext,
        entity: &str,
        _action: OperationType,
    ) -> GuardAction {
        match entity {
            "FilmCategory" => GuardAction::Block(None),
            _ => GuardAction::Allow,
        }
    }

    fn field_guard(
        &self,
        _ctx: &ResolverContext,
        entity: &str,
        field: &str,
        action: OperationType,
    ) -> GuardAction {
        match (entity, field, action) {
            ("Language", "lastUpdate", _) => GuardAction::Block(None),
            ("Language", "name", OperationType::Update) => GuardAction::Block(None),
            _ => GuardAction::Allow,
        }
    }
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
    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .data(database)
        .finish()
}

pub async fn get_schema() -> Schema {
    let database = Database::connect("sqlite://sakila.db").await.unwrap();
    let schema = schema(database.unrestricted(), None, None).unwrap();

    schema
}

pub fn assert_eq(a: Response, b: &str) {
    assert_eq!(
        a.data.into_json().unwrap(),
        serde_json::from_str::<serde_json::Value>(b).unwrap()
    )
}

#[tokio::test]
async fn entity_guard() {
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
    let schema = get_schema().await;

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
