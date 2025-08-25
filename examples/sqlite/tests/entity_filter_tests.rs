use async_graphql::{dynamic::*, Response};
use sea_orm::{ColumnTrait, Condition, Database, DatabaseConnection};
use seaography::{
    async_graphql, lazy_static, Builder, BuilderContext, LifecycleHooks, LifecycleHooksInterface,
    OperationType,
};
use seaography_sqlite_example::entities::*;
use serde_json::json;

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
            "Customer" | "Customers" => Some(Condition::all().add(customer::Column::StoreId.eq(2))),
            "Inventory" => Some(Condition::all().add(inventory::Column::StoreId.eq(2))),
            "Staff" => Some(Condition::all().add(staff::Column::StoreId.eq(2))),
            "Store" | "Stores" => Some(Condition::all().add(store::Column::StoreId.eq(2))),
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

    #[cfg(feature = "field-pluralize")]
    let stores_name = "stores";
    #[cfg(not(feature = "field-pluralize"))]
    let stores_name = "store";

    let query = format!(
        "
      {{
        {stores_name} {{
          nodes {{
            storeId
            address {{
              address
            }}
            manager {{
              storeId
              firstName
              lastName
            }}
          }}
        }}
      }}
    "
    );
    let response = schema.execute(query).await;
    assert_eq!(response.errors.len(), 0);
    assert_eq!(
        response.data.into_json().unwrap(),
        json!({
            stores_name: {
                "nodes": [{
                    "storeId": 2,
                    "address": {
                        "address": "28 MySQL Boulevard"
                    },
                    "manager": {
                        "storeId": 2,
                        "firstName": "Jon",
                        "lastName": "Stephens"
                    }
                }]
            }
        })
    );

    #[cfg(feature = "field-pluralize")]
    let customers_name = "customers";
    #[cfg(not(feature = "field-pluralize"))]
    let customers_name = "customer";

    let query = format!(
        "
      {{
        {customers_name}(pagination: {{ page: {{ page: 0, limit: 2 }} }}) {{
          nodes {{
            storeId
            customerId
            firstName
            lastName
          }}
          paginationInfo {{
            pages
            current
          }}
        }}
      }}
    "
    );

    let response = schema.execute(query).await;
    assert_eq!(response.errors.len(), 0);
    assert_eq!(
        response.data.into_json().unwrap(),
        json!({
            customers_name: {
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
        }),
    );

    let query = format!(
        "
      {{
        {stores_name} {{
          nodes {{
            storeId
            address {{
              address
            }}
            {customers_name}(pagination: {{ page: {{ page: 0, limit: 2 }} }}) {{
              nodes {{
                storeId
                firstName
                lastName
              }}
            }}
          }}
        }}
      }}
    "
    );
    let response = schema.execute(query).await;
    assert_eq!(response.errors.len(), 0);
    assert_eq!(
        response.data.into_json().unwrap(),
        json!(
        {
            stores_name: {
              "nodes": [
                {
                  "storeId": 2,
                  "address": {
                    "address": "28 MySQL Boulevard"
                  },
                  customers_name: {
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
        }),
    );
}
