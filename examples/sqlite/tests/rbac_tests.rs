use async_graphql::{dynamic::*, Response};
use sea_orm::Database;
use seaography::{async_graphql, DatabaseContext};

pub async fn get_schema() -> Schema {
    let database = Database::connect("sqlite://sakila.db").await.unwrap();
    let schema =
        seaography_sqlite_example::query_root::schema(database.unrestricted(), None, None).unwrap();

    schema
}

pub fn assert_eq(a: Response, b: &str) {
    assert_eq!(
        a.data.into_json().unwrap(),
        serde_json::from_str::<serde_json::Value>(b).unwrap()
    )
}

#[tokio::test]
async fn test_sea_orm_current_user_role_permissions() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                  _sea_orm_current_user_role_permissions {
                    role {
                      role
                    }
                    permissions {
                      resource {
                        schema
                        table
                      }
                      permission {
                        action
                      }
                    }
                  }
                }
                "#,
            )
            .await,
        r#"
        {
          "_sea_orm_current_user_role_permissions": {
            "role": {
              "role": "unrestricted"
            },
            "permissions": [
              {
                "resource": {
                  "schema": null,
                  "table": "*"
                },
                "permission": {
                  "action": "*"
                }
              }
            ]
          }
        }
        "#,
    )
}

#[tokio::test]
async fn test_sea_orm_roles_and_ranks() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                  _sea_orm_roles_and_ranks {
                    role {
                      id
                      role
                    }
                    rank
                  }
                }
                "#,
            )
            .await,
        r#"
        {
          "_sea_orm_roles_and_ranks": [
            {
              "role": {
                "id": 1,
                "role": "unrestricted"
              },
              "rank": 1
            }
          ]
        }
        "#,
    )
}

#[tokio::test]
async fn test_sea_orm_role_hierarchy_edges() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                  _sea_orm_role_hierarchy_edges(role_id: 1) {
                    superRoleId
                    roleId
                  }
                }
                "#,
            )
            .await,
        r#"
        {
          "_sea_orm_role_hierarchy_edges": []
        }
        "#,
    )
}

#[tokio::test]
async fn test_sea_orm_role_permissions_by_resources() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                  _sea_orm_role_permissions_by_resources(role_id: 1) {
                    resource {
                      id
                      schema
                      table
                    }
                    permissions {
                      permission {
                        id
                        action
                      }
                      grant
                    }
                  }
                }
                "#,
            )
            .await,
        r#"
        {
          "_sea_orm_role_permissions_by_resources": [
            {
              "resource": {
                "id": 1,
                "schema": null,
                "table": "*"
              },
              "permissions": [
                {
                  "permission": {
                    "id": 1,
                    "action": "*"
                  },
                  "grant": true
                }
              ]
            }
          ]
        }
        "#,
    )
}
