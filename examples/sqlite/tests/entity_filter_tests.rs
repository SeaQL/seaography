use async_graphql::dynamic::*;
use sea_orm::{ColumnTrait, Condition, Database};
use seaography::{
    async_graphql, lazy_static, BuilderContext, LifecycleHooks, LifecycleHooksInterface,
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

async fn schema() -> Schema {
    let database = Database::connect("sqlite://sakila.db").await.unwrap();
    seaography_sqlite_example::query_root::schema_builder(&CONTEXT, database, None, None)
        .finish()
        .unwrap()
}

#[tokio::test]
async fn only_store_2() {
    let schema = schema().await;

    let stores_name = if cfg!(feature = "field-pluralize") {
        "stores"
    } else {
        "store"
    };

    let staff_name = if cfg!(feature = "field-pluralize") {
        "staff_single"
    } else {
        "staff"
    };

    let query = format!(
        "
      {{
        {stores_name} {{
          nodes {{
            storeId
            address {{
              address
            }}
            {staff_name} {{
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
                    staff_name: {
                        "storeId": 2,
                        "firstName": "Jon",
                        "lastName": "Stephens"
                    }
                }]
            }
        })
    );

    let customers_name = if cfg!(feature = "field-pluralize") {
        "customers"
    } else {
        "customer"
    };

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
