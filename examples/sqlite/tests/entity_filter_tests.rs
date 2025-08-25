use async_graphql::{dynamic::*, Response};
use sea_orm::{ColumnTrait, Condition, Database, DatabaseConnection};
use seaography::{
    async_graphql, lazy_static, Builder, BuilderContext, LifecycleHooks, LifecycleHooksInterface,
    OperationType,
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
    fn entity_filter(
        &self,
        _ctx: &ResolverContext,
        entity: &str,
        _action: OperationType,
    ) -> Option<Condition> {
        match entity {
            "Customer" => Some(Condition::all().add(customer::Column::StoreId.eq(2))),
            "Inventory" => Some(Condition::all().add(inventory::Column::StoreId.eq(2))),
            "Staff" => Some(Condition::all().add(staff::Column::StoreId.eq(2))),
            "Store" => Some(Condition::all().add(store::Column::StoreId.eq(2))),
            _ => None,
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
async fn only_store_2() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                  store {
                    nodes {
                      storeId
                      address {
                        address
                      }
                      manager {
                        storeId
                        firstName
                        lastName
                      }
                    }
                  }
                }
                "#,
            )
            .await,
        r#"
        {
            "store": {
              "nodes": [
                {
                  "storeId": 2,
                  "address": {
                    "address": "28 MySQL Boulevard"
                },
                  "manager": {
                    "storeId": 2,
                    "firstName": "Jon",
                    "lastName": "Stephens"
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
                  customer(pagination: { page: { page: 0, limit: 2 } }) {
                    nodes {
                      storeId
                      customerId
                      firstName
                      lastName
                    }
                    paginationInfo {
                      pages
                      current
                    }
                  }
                }
                "#,
            )
            .await,
        r#"
        {
            "customer": {
              "nodes": [
                {
                  "storeId": 2,
                  "customerId": 4,
                  "firstName": "BARBARA",
                  "lastName": "JONES"
                },
                {
                  "storeId": 2,
                  "customerId": 6,
                  "firstName": "JENNIFER",
                  "lastName": "DAVIS"
                }
              ],
              "paginationInfo": {
                "pages": 137,
                "current": 0
              }
            }
        }
        "#,
    );

    assert_eq(
        schema
            .execute(
                r#"
                {
                  store {
                    nodes {
                      storeId
                      address {
                        address
                    }
                    customer(pagination: { page: { page: 0, limit: 2 } }) {
                      nodes {
                        storeId
                        firstName
                        lastName
                      }
                    }
                  }
                }
                }
                "#,
            )
            .await,
        r#"
        {
            "store": {
              "nodes": [
                {
                  "storeId": 2,
                  "address": {
                    "address": "28 MySQL Boulevard"
                  },
                  "customer": {
                    "nodes": [
                      {
                        "storeId": 2,
                        "firstName": "BARBARA",
                        "lastName": "JONES"
                      },
                      {
                        "storeId": 2,
                        "firstName": "JENNIFER",
                        "lastName": "DAVIS"
                      }
                    ]
                  }
                }
              ]
            }
        }
        "#,
    );
}
