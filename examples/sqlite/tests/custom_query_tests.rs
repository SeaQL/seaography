use async_graphql::{dynamic::*, Request, Response, Variables};
use sea_orm::Database;
use seaography::async_graphql;
use serde::Deserialize;
use serde_json::json;

pub async fn get_schema() -> Schema {
    let database = Database::connect("sqlite://sakila.db").await.unwrap();
    let schema = seaography_sqlite_example::query_root::schema(database, None, None).unwrap();

    schema
}

pub fn assert_eq(a: Response, b: &str) {
    assert_eq!(
        a.data.into_json().unwrap(),
        serde_json::from_str::<serde_json::Value>(b).unwrap()
    )
}

#[tokio::test]
async fn test_custom_query_with_pagination() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                  customer_of_store2(pagination: { page: { page: 0, limit: 2 } }) {
                    nodes {
                      storeId
                      customerId
                      lastName
                      email
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
          "customer_of_store2": {
            "nodes": [
              {
                "storeId": 2,
                "customerId": 4,
                "lastName": "JONES",
                "email": "BARBARA.JONES@sakilacustomer.org"
              },
              {
                "storeId": 2,
                "customerId": 6,
                "lastName": "DAVIS",
                "email": "JENNIFER.DAVIS@sakilacustomer.org"
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
                  customer_of_store2(pagination: { page: { page: 1, limit: 1 } }) {
                    nodes {
                      storeId
                      customerId
                      lastName
                      email
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
          "customer_of_store2": {
            "nodes": [
              {
                "storeId": 2,
                "customerId": 6,
                "lastName": "DAVIS",
                "email": "JENNIFER.DAVIS@sakilacustomer.org"
              }
            ],
            "paginationInfo": {
              "pages": 273,
              "current": 1
            }
          }
        }
        "#,
    );
}

#[tokio::test]
async fn test_custom_query_many() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                  many_rental {
                    rentalId
                  }
                }
                "#,
            )
            .await,
        r#"
        {
          "many_rental": [
            { "rentalId": 1 },
            { "rentalId": 2 },
            { "rentalId": 3 },
            { "rentalId": 4 },
            { "rentalId": 5 },
            { "rentalId": 6 },
            { "rentalId": 7 },
            { "rentalId": 8 },
            { "rentalId": 9 },
            { "rentalId": 10 }
          ]
        }
        "#,
    );
}

#[tokio::test]
async fn test_custom_query_with_no_pagination() {
    let schema = get_schema().await;

    let json = schema
        .execute(
            r#"
            {
                customer_of_store2 {
                nodes {
                    storeId
                    customerId
                    lastName
                    email
                }
                paginationInfo {
                    pages
                    current
                }
                }
            }
            "#,
        )
        .await
        .data
        .into_json()
        .unwrap();

    let parsed: QueryResult = serde_json::from_value(json).unwrap();

    assert_eq!(parsed.customer_of_store2.nodes.len(), 273);
    assert_eq!(parsed.customer_of_store2.pagination_info.pages, 1);
    assert_eq!(parsed.customer_of_store2.pagination_info.current, 1);

    #[derive(Deserialize)]
    struct QueryResult {
        customer_of_store2: CustomerOfStore2,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct CustomerOfStore2 {
        nodes: Vec<Customer>,
        pagination_info: PaginationInfo,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Customer {
        store_id: i32,
        customer_id: i32,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct PaginationInfo {
        pages: i32,
        current: i32,
    }
}

#[tokio::test]
async fn option_entity_object() {
    let schema = get_schema().await;

    let query = r#"
        query($staffId: Int!) {
            staff_by_id(id: $staffId) {
                firstName
                lastName
            }
        }
    "#;

    let vars = Variables::from_json(json!({ "staffId": 2 }));
    let response = schema.execute(Request::new(query).variables(vars)).await;
    assert_eq!(response.errors.len(), 0);
    assert_eq!(
        response.data.into_json().unwrap(),
        json!({
            "staff_by_id": {
                "firstName": "Jon",
                "lastName": "Stephens",
            }
        })
    );

    let vars = Variables::from_json(json!({ "staffId": 0 }));
    let response = schema.execute(Request::new(query).variables(vars)).await;
    assert_eq!(response.errors.len(), 0);
    assert_eq!(
        response.data.into_json().unwrap(),
        json!({
            "staff_by_id": null
        })
    );
}

#[tokio::test]
async fn option_entity_object_relation_owner() {
    let schema = get_schema().await;

    let query = r#"
        query($staffId: Int!) {
            staff_by_id(id: $staffId) {
                selfRefReverse {
                    nodes {
                        firstName
                    }
                }
            }
        }
    "#;

    let vars = Variables::from_json(json!({ "staffId": 1 }));
    let response = schema.execute(Request::new(query).variables(vars)).await;
    assert_eq!(response.errors.len(), 0);
    assert_eq!(
        response.data.into_json().unwrap(),
        json!({
            "staff_by_id": {
                "selfRefReverse": {
                    "nodes": [{
                        "firstName": "Jon"
                    }]
                }
            }
        })
    );

    let vars = Variables::from_json(json!({ "staffId": 2 }));
    let response = schema.execute(Request::new(query).variables(vars)).await;
    assert_eq!(response.errors.len(), 0);
    assert_eq!(
        response.data.into_json().unwrap(),
        json!({
            "staff_by_id": {
                "selfRefReverse": {
                    "nodes": []
                }
            }
        })
    );

    let vars = Variables::from_json(json!({ "staffId": 0 }));
    let response = schema.execute(Request::new(query).variables(vars)).await;
    assert_eq!(response.errors.len(), 0);
    assert_eq!(
        response.data.into_json().unwrap(),
        json!({
            "staff_by_id": null
        })
    );
}

#[tokio::test]
async fn option_entity_object_relation_not_owner() {
    let schema = get_schema().await;

    let query = r#"
        query($staffId: Int!) {
            staff_by_id(id: $staffId) {
                selfRef {
                    firstName
                }
            }
        }
    "#;

    let vars = Variables::from_json(json!({ "staffId": 1 }));
    let response = schema.execute(Request::new(query).variables(vars)).await;
    assert_eq!(response.errors.len(), 0);
    assert_eq!(
        response.data.into_json().unwrap(),
        json!({
            "staff_by_id": {
                "selfRef": null
            }
        })
    );

    let vars = Variables::from_json(json!({ "staffId": 2 }));
    let response = schema.execute(Request::new(query).variables(vars)).await;
    assert_eq!(response.errors.len(), 0);
    assert_eq!(
        response.data.into_json().unwrap(),
        json!({
            "staff_by_id": {
                "selfRef": {
                    "firstName": "Mike"
                }
            }
        })
    );

    let vars = Variables::from_json(json!({ "staffId": 0 }));
    let response = schema.execute(Request::new(query).variables(vars)).await;
    assert_eq!(response.errors.len(), 0);
    assert_eq!(
        response.data.into_json().unwrap(),
        json!({
            "staff_by_id": null
        })
    );
}

#[tokio::test]
async fn option_entity_object_via_relation_owner() {
    let schema = get_schema().await;

    let query = r#"
        query($staffId: Int!) {
            staff_by_id(id: $staffId) {
                rental(
                    orderBy: { rentalId: ASC },
                    pagination: { page: { page: 0, limit: 1 } }
                ) {
                    nodes {
                        rentalId
                    }
                }
            }
        }
    "#;

    let vars = Variables::from_json(json!({ "staffId": 2 }));
    let response = schema.execute(Request::new(query).variables(vars)).await;
    assert_eq!(response.errors.len(), 0);
    assert_eq!(
        response.data.into_json().unwrap(),
        json!({
            "staff_by_id": {
                "rental": {
                    "nodes": [{
                        "rentalId": 4
                    }]
                }
            }
        })
    );

    let vars = Variables::from_json(json!({ "staffId": 0 }));
    let response = schema.execute(Request::new(query).variables(vars)).await;
    assert_eq!(response.errors.len(), 0);
    assert_eq!(
        response.data.into_json().unwrap(),
        json!({
            "staff_by_id": null
        })
    );
}

#[tokio::test]
async fn option_entity_object_via_relation_not_owner() {
    let schema = get_schema().await;

    let query = r#"
        query($staffId: Int!) {
            staff_by_id(id: $staffId) {
                store {
                    storeId
                }
            }
        }
    "#;

    let vars = Variables::from_json(json!({ "staffId": 1 }));
    let response = schema.execute(Request::new(query).variables(vars)).await;
    assert_eq!(response.errors.len(), 0);
    assert_eq!(
        response.data.into_json().unwrap(),
        json!({
            "staff_by_id": {
                "store": {
                    "storeId": 1
                }
            }
        })
    );

    let vars = Variables::from_json(json!({ "staffId": 0 }));
    let response = schema.execute(Request::new(query).variables(vars)).await;
    assert_eq!(response.errors.len(), 0);
    assert_eq!(
        response.data.into_json().unwrap(),
        json!({
            "staff_by_id": null
        })
    );
}
