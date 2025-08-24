use async_graphql::{
    dynamic::{FieldValue, ResolverContext, Schema, SchemaError},
    Request, Response, Variables,
};
use sea_orm::{Database, DatabaseConnection};
use seaography::{
    async_graphql, lazy_static, Builder, BuilderContext, GuardAction, LifecycleHooks,
    LifecycleHooksInterface, OperationType,
};
use seaography_sqlite_example::entities::*;
use serde_json::json;
use std::{
    collections::BTreeSet,
    sync::{Arc, Mutex},
};

lazy_static::lazy_static! {
    static ref CONTEXT : BuilderContext = {
        BuilderContext {
            hooks: LifecycleHooks::new(MyHooks),
            ..Default::default()
        }
    };
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HookCall {
    EntityGuard(String, OperationType),
    FieldGuard(String, Option<i64>, String, OperationType),
}

fn id_from_entity(entity: &str, value: &FieldValue<'_>) -> Option<i64> {
    match entity {
        "Actor" => value
            .downcast_ref::<actor::Model>()
            .map(|actor| actor.actor_id as i64),
        "Rental" => value
            .downcast_ref::<rental::Model>()
            .map(|rental| rental.rental_id as i64),
        "Staff" => value
            .downcast_ref::<staff::Model>()
            .map(|staff| staff.staff_id as i64),
        _ => None,
    }
}

impl HookCall {
    fn entity_guard(entity: impl Into<String>, action: OperationType) -> Self {
        HookCall::EntityGuard(entity.into(), action)
    }

    fn field_guard(
        entity: impl Into<String>,
        id: Option<i64>,
        field: impl Into<String>,
        action: OperationType,
    ) -> Self {
        HookCall::FieldGuard(entity.into(), id, field.into(), action)
    }
}

#[derive(Default, Clone)]
struct Log {
    calls: Arc<Mutex<Vec<HookCall>>>,
}

#[derive(Default, Clone)]
struct Permissions {
    actors: BTreeSet<i32>,
    staff: BTreeSet<i16>,
}

#[derive(Clone)]
struct MyHooks;

impl Log {
    fn record(&self, call: HookCall) {
        self.calls.lock().unwrap().push(call);
    }

    fn calls(&self) -> Vec<HookCall> {
        let mut calls = self.calls.lock().unwrap().clone();
        // Sort the list of calls so the tests don't depend on the order of field resolution
        calls.sort();
        calls
    }
}

impl LifecycleHooksInterface for MyHooks {
    fn entity_guard(
        &self,
        ctx: &ResolverContext,
        entity: &str,
        action: OperationType,
    ) -> GuardAction {
        ctx.data::<Log>()
            .unwrap()
            .record(HookCall::entity_guard(entity, action));
        match entity {
            "FilmCategory" => GuardAction::Block(None),
            _ => GuardAction::Allow,
        }
    }

    fn field_guard(
        &self,
        ctx: &ResolverContext,
        entity: &str,
        field: &str,
        action: OperationType,
    ) -> GuardAction {
        ctx.data::<Log>().unwrap().record(HookCall::field_guard(
            entity,
            id_from_entity(entity, ctx.parent_value),
            field,
            action,
        ));
        let permissions = ctx.data::<Permissions>().unwrap();
        match (entity, field, action) {
            ("Language", "lastUpdate", _) => GuardAction::Block(None),
            ("Language", "name", OperationType::Update) => GuardAction::Block(None),
            ("Actor", _, _) => {
                let Some(actor) = ctx.parent_value.downcast_ref::<actor::Model>() else {
                    return GuardAction::Block(Some("downcast_ref failed".into()));
                };

                if permissions.actors.contains(&actor.actor_id) {
                    GuardAction::Allow
                } else {
                    GuardAction::Block(Some(format!(
                        "{:?} on actor {} denied",
                        action, actor.actor_id
                    )))
                }
            }
            ("Staff", _, _) => {
                let Some(staff) = ctx.parent_value.downcast_ref::<staff::Model>() else {
                    return GuardAction::Block(Some("downcast_ref failed".into()));
                };

                if permissions.staff.contains(&staff.staff_id) {
                    GuardAction::Allow
                } else {
                    GuardAction::Block(Some(format!(
                        "{:?} on Staff({}).{} denied",
                        action, staff.staff_id, field,
                    )))
                }
            }
            _ => GuardAction::Allow,
        }
    }
}

fn schema(
    database: DatabaseConnection,
    log: Log,
    permissions: Permissions,
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
        .data(log)
        .data(permissions)
        .finish()
}

async fn get_schema(permissions: Permissions) -> (Schema, Log) {
    let log = Log::default();
    let database = Database::connect("sqlite://sakila.db").await.unwrap();
    let schema = schema(database, log.clone(), permissions, None, None).unwrap();

    (schema, log)
}

pub fn assert_eq(a: Response, b: &str) {
    assert_eq!(
        a.data.into_json().unwrap(),
        serde_json::from_str::<serde_json::Value>(b).unwrap()
    )
}

#[tokio::test]
async fn entity_guard() {
    let permissions = Permissions::default();
    let (schema, log) = get_schema(permissions).await;

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
    assert_eq!(response.data.into_json().unwrap(), serde_json::Value::Null);
    assert_eq!(
        log.calls(),
        vec![
            HookCall::entity_guard("FilmCategory", OperationType::Read),
            HookCall::entity_guard("Language", OperationType::Read),
            HookCall::field_guard("Language", None, "languageId", OperationType::Read),
            HookCall::field_guard("Language", None, "languageId", OperationType::Read),
            HookCall::field_guard("Language", None, "languageId", OperationType::Read),
            HookCall::field_guard("Language", None, "languageId", OperationType::Read),
            HookCall::field_guard("Language", None, "languageId", OperationType::Read),
            HookCall::field_guard("Language", None, "languageId", OperationType::Read),
            HookCall::field_guard("Language", None, "name", OperationType::Read),
            HookCall::field_guard("Language", None, "name", OperationType::Read),
            HookCall::field_guard("Language", None, "name", OperationType::Read),
            HookCall::field_guard("Language", None, "name", OperationType::Read),
            HookCall::field_guard("Language", None, "name", OperationType::Read),
            HookCall::field_guard("Language", None, "name", OperationType::Read),
        ]
    );
}

#[tokio::test]
async fn field_guard() {
    let permissions = Permissions::default();
    let (schema, log) = get_schema(permissions).await;

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
    assert_eq!(response.data.into_json().unwrap(), serde_json::Value::Null);
    assert_eq!(
        log.calls(),
        vec![
            HookCall::entity_guard("Language", OperationType::Read),
            HookCall::field_guard("Language", None, "languageId", OperationType::Read),
            HookCall::field_guard("Language", None, "lastUpdate", OperationType::Read),
            HookCall::field_guard("Language", None, "name", OperationType::Read),
        ]
    );
}

#[tokio::test]
async fn field_guard_mutation() {
    let permissions = Permissions::default();
    let (schema, log) = get_schema(permissions).await;

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
    assert_eq!(response.data.into_json().unwrap(), serde_json::Value::Null);
    assert_eq!(
        log.calls(),
        vec![
            HookCall::entity_guard("Language", OperationType::Update),
            HookCall::field_guard("Language", None, "name", OperationType::Update),
        ]
    );
}

#[tokio::test]
async fn permissions() {
    let mut permissions = Permissions::default();
    permissions.actors.insert(3);
    let (schema, _log) = get_schema(permissions).await;
    let query = r#"
        query ($actorId: Int!) {
            actor(filters: { actorId: { eq: $actorId } }) {
                nodes {
                    firstName
                    lastName
                }
            }
        }
    "#;

    let vars = Variables::from_json(json!({ "actorId": 3 }));
    let response = schema.execute(Request::new(query).variables(vars)).await;
    assert_eq!(response.errors.len(), 0);
    assert_eq!(
        response.data.into_json().unwrap(),
        json!({
            "actor": { "nodes": [{ "firstName": "ED", "lastName": "CHASE" }] }
        })
    );

    let vars = Variables::from_json(json!({ "actorId": 4 }));
    let response = schema.execute(Request::new(query).variables(vars)).await;
    assert_eq!(response.errors.len(), 1);
    assert_eq!(response.errors[0].message, "Read on actor 4 denied");
    assert_eq!(response.data.into_json().unwrap(), serde_json::Value::Null);
}

fn entity_object_relation_owner_query() -> String {
    r#"
        query($staffId: Int!) {
            staff(filters: { staffId: { eq: $staffId }}) {
                nodes {
                    selfRefReverse {
                        nodes {
                            firstName
                        }
                    }
                }
            }
        }
    "#
    .to_string()
}

#[tokio::test]
async fn entity_object_relation_owner1() {
    let mut permissions = Permissions::default();
    permissions.staff.insert(1);
    permissions.staff.insert(2);
    let (schema, log) = get_schema(permissions).await;
    let query = entity_object_relation_owner_query();
    let vars = Variables::from_json(json!({ "staffId": 1 }));
    let response = schema.execute(Request::new(&query).variables(vars)).await;
    assert_eq!(response.errors.len(), 0);
    assert_eq!(
        response.data.into_json().unwrap(),
        json!({
            "staff": {
                "nodes": [{
                    "selfRefReverse": {
                        "nodes": [{
                            "firstName": "Jon"
                        }]
                    }
                }]
            }
        })
    );
    assert_eq!(
        log.calls(),
        vec![
            HookCall::entity_guard("Staff", OperationType::Read),
            HookCall::entity_guard("Staff", OperationType::Read),
            HookCall::field_guard("Staff", Some(1), "selfRefReverse", OperationType::Read),
            HookCall::field_guard("Staff", Some(2), "firstName", OperationType::Read),
        ]
    );
}

#[tokio::test]
async fn entity_object_relation_owner2() {
    let mut permissions = Permissions::default();
    permissions.staff.insert(1);
    permissions.staff.insert(2);
    let (schema, log) = get_schema(permissions).await;
    let query = entity_object_relation_owner_query();
    let vars = Variables::from_json(json!({ "staffId": 2 }));
    let response = schema.execute(Request::new(&query).variables(vars)).await;
    assert_eq!(response.errors.len(), 0);
    assert_eq!(
        response.data.into_json().unwrap(),
        json!({
            "staff": {
                "nodes": [{
                    "selfRefReverse": {
                        "nodes": []
                    }
                }]
            }
        })
    );
    assert_eq!(
        log.calls(),
        vec![
            HookCall::entity_guard("Staff", OperationType::Read),
            HookCall::entity_guard("Staff", OperationType::Read),
            HookCall::field_guard("Staff", Some(2), "selfRefReverse", OperationType::Read),
        ]
    );
}

#[tokio::test]
async fn entity_object_relation_owner3() {
    // Check that access to selfRefReverse is denied in the case where it would otherwise
    // be non-empty
    let mut permissions = Permissions::default();
    permissions.staff.insert(2);
    let (schema, _log) = get_schema(permissions).await;
    let query = entity_object_relation_owner_query();
    let vars = Variables::from_json(json!({ "staffId": 1 }));
    let response = schema.execute(Request::new(&query).variables(vars)).await;
    assert_eq!(response.errors.len(), 1);
    assert_eq!(
        response.errors[0].message,
        "Read on Staff(1).selfRefReverse denied"
    );
    assert_eq!(response.data.into_json().unwrap(), serde_json::Value::Null);
}

#[tokio::test]
async fn entity_object_relation_owner4() {
    // Check that access to selfRefReverse is denied in the case where it would otherwise
    // be an empty list
    let permissions = Permissions::default();
    let (schema, _log) = get_schema(permissions).await;
    let query = entity_object_relation_owner_query();
    let vars = Variables::from_json(json!({ "staffId": 2 }));
    let response = schema.execute(Request::new(&query).variables(vars)).await;
    assert_eq!(response.errors.len(), 1);
    assert_eq!(
        response.errors[0].message,
        "Read on Staff(2).selfRefReverse denied"
    );
    assert_eq!(response.data.into_json().unwrap(), serde_json::Value::Null);
}

fn entity_object_relation_not_owner_query() -> String {
    r#"
        query($staffId: Int!) {
            staff(filters: { staffId: { eq: $staffId }}) {
                nodes {
                    selfRef {
                        firstName
                    }
                }
            }
        }
    "#
    .to_string()
}

#[tokio::test]
async fn entity_object_relation_not_owner1() {
    let mut permissions = Permissions::default();
    permissions.staff.insert(1);
    permissions.staff.insert(2);
    let (schema, log) = get_schema(permissions).await;
    let query = entity_object_relation_not_owner_query();
    let vars = Variables::from_json(json!({ "staffId": 1 }));
    let response = schema.execute(Request::new(&query).variables(vars)).await;
    assert_eq!(response.errors.len(), 0);
    assert_eq!(
        response.data.into_json().unwrap(),
        json!({
            "staff": {
                "nodes": [{
                    "selfRef": null,
                }]
            }
        })
    );
    assert_eq!(
        log.calls(),
        vec![
            HookCall::entity_guard("Staff", OperationType::Read),
            HookCall::entity_guard("Staff", OperationType::Read),
            HookCall::field_guard("Staff", Some(1), "selfRef", OperationType::Read),
        ]
    );
}

#[tokio::test]
async fn entity_object_relation_not_owner2() {
    let mut permissions = Permissions::default();
    permissions.staff.insert(1);
    permissions.staff.insert(2);
    let (schema, log) = get_schema(permissions).await;
    let query = entity_object_relation_not_owner_query();
    let vars = Variables::from_json(json!({ "staffId": 2 }));
    let response = schema.execute(Request::new(&query).variables(vars)).await;
    assert_eq!(response.errors.len(), 0);
    assert_eq!(
        response.data.into_json().unwrap(),
        json!({
            "staff": {
                "nodes": [{
                    "selfRef": {
                        "firstName": "Mike",
                    },
                }]
            }
        })
    );
    assert_eq!(
        log.calls(),
        vec![
            HookCall::entity_guard("Staff", OperationType::Read),
            HookCall::entity_guard("Staff", OperationType::Read),
            HookCall::field_guard("Staff", Some(1), "firstName", OperationType::Read),
            HookCall::field_guard("Staff", Some(2), "selfRef", OperationType::Read),
        ]
    );
}

#[tokio::test]
async fn entity_object_relation_not_owner3() {
    let mut permissions = Permissions::default();
    permissions.staff.insert(2);
    let (schema, _log) = get_schema(permissions).await;
    let query = entity_object_relation_not_owner_query();
    let vars = Variables::from_json(json!({ "staffId": 1 }));
    let response = schema.execute(Request::new(&query).variables(vars)).await;
    assert_eq!(response.errors.len(), 1);
    assert_eq!(
        response.errors[0].message,
        "Read on Staff(1).selfRef denied"
    );
    assert_eq!(response.data.into_json().unwrap(), serde_json::Value::Null);
}

#[tokio::test]
async fn entity_object_relation_not_owner4() {
    let mut permissions = Permissions::default();
    permissions.staff.insert(1);
    let (schema, _log) = get_schema(permissions).await;
    let query = entity_object_relation_not_owner_query();
    let vars = Variables::from_json(json!({ "staffId": 2 }));
    let response = schema.execute(Request::new(&query).variables(vars)).await;
    assert_eq!(response.errors.len(), 1);
    assert_eq!(
        response.errors[0].message,
        "Read on Staff(2).selfRef denied"
    );
    assert_eq!(response.data.into_json().unwrap(), serde_json::Value::Null);
}

fn entity_object_via_relation_owner_query() -> String {
    r#"
        query($staffId: Int!) {
            staff(filters: { staffId: { eq: $staffId }}) {
                nodes {
                    rental(
                        orderBy: { rentalId: ASC }
                        pagination: { page: { page: 0 limit: 1 } }
                    ) {
                        nodes {
                            rentalId
                        }
                    }
                }
            }
        }
    "#
    .to_string()
}

#[tokio::test]
async fn entity_object_via_relation_owner1() {
    let mut permissions = Permissions::default();
    permissions.staff.insert(2);
    let (schema, log) = get_schema(permissions).await;
    let query = entity_object_via_relation_owner_query();
    let vars = Variables::from_json(json!({ "staffId": 2 }));
    let response = schema.execute(Request::new(&query).variables(vars)).await;
    assert_eq!(response.errors.len(), 0);
    let data = response.data.into_json().unwrap();
    assert_eq!(
        data,
        json!({
            "staff": {
                "nodes": [{
                    "rental": {
                        "nodes": [{
                            "rentalId": 4
                        }]
                    }
                }]
            }
        })
    );
    assert_eq!(
        log.calls(),
        vec![
            HookCall::entity_guard("Rental", OperationType::Read),
            HookCall::entity_guard("Staff", OperationType::Read),
            HookCall::field_guard("Rental", Some(4), "rentalId", OperationType::Read),
            HookCall::field_guard("Staff", Some(2), "rental", OperationType::Read),
        ]
    );
}

#[tokio::test]
async fn entity_object_via_relation_owner2() {
    let mut permissions = Permissions::default();
    permissions.staff.insert(1);
    let (schema, _log) = get_schema(permissions).await;
    let query = entity_object_via_relation_owner_query();
    let vars = Variables::from_json(json!({ "staffId": 2 }));
    let response = schema.execute(Request::new(&query).variables(vars)).await;
    assert_eq!(response.errors.len(), 1);
    assert_eq!(response.errors[0].message, "Read on Staff(2).rental denied");
    assert_eq!(response.data.into_json().unwrap(), serde_json::Value::Null);
}

fn entity_object_via_relation_not_owner_query() -> String {
    r#"
        query($staffId: Int!) {
            staff(filters: { staffId: { eq: $staffId }}) {
                nodes {
                    store {
                        storeId
                    }
                }
            }
        }
    "#
    .to_string()
}

#[tokio::test]
async fn entity_object_via_relation_not_owner1() {
    let mut permissions = Permissions::default();
    permissions.staff.insert(1);
    let (schema, log) = get_schema(permissions).await;
    let query = entity_object_via_relation_not_owner_query();
    let vars = Variables::from_json(json!({ "staffId": 1 }));
    let response = schema.execute(Request::new(&query).variables(vars)).await;
    assert_eq!(response.errors.len(), 0);
    assert_eq!(
        response.data.into_json().unwrap(),
        json!({
            "staff": {
                "nodes": [{
                    "store": {
                        "storeId": 1
                    }
                }]
            }
        })
    );
    assert_eq!(
        log.calls(),
        vec![
            HookCall::entity_guard("Staff", OperationType::Read),
            HookCall::entity_guard("Store", OperationType::Read),
            HookCall::field_guard("Staff", Some(1), "store", OperationType::Read),
            HookCall::field_guard("Store", None, "storeId", OperationType::Read),
        ]
    );
}

#[tokio::test]
async fn entity_object_via_relation_not_owner2() {
    let mut permissions = Permissions::default();
    permissions.staff.insert(2);
    let (schema, _log) = get_schema(permissions).await;
    let query = entity_object_via_relation_not_owner_query();
    let vars = Variables::from_json(json!({ "staffId": 1 }));
    let response = schema.execute(Request::new(&query).variables(vars)).await;
    assert_eq!(response.errors.len(), 1);
    assert_eq!(response.errors[0].message, "Read on Staff(1).store denied");
    assert_eq!(response.data.into_json().unwrap(), serde_json::Value::Null);
}
