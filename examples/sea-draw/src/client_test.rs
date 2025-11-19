use sea_orm::entity::prelude::Uuid;
use serde_json::json;
use std::time::Duration;
use tracing::instrument;

const OUTPUT1_EXPECTED: &str = include_str!("../fixtures/output1.svg");
const OUTPUT2_EXPECTED: &str = include_str!("../fixtures/output2.svg");

use crate::client::{
    client::Client,
    graphql::{graphql, unauthenticated},
};

#[instrument(skip_all)]
pub async fn client_test(
    address: String,
    port: u16,
    root_account_id: Uuid,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("http://{}:{}", address, port);
    wait_for_server(&url).await?;

    tracing::info!("Client test: url = {}", url);
    tracing::info!("root token = {}", root_account_id);

    test_permissions(&url, root_account_id).await;
    test_entities(&url, root_account_id).await;
    test_boolean_operators(&url, root_account_id).await;

    tracing::info!("All tests completed successfully!");

    Ok(())
}

#[instrument(skip_all)]
async fn test_entities(url: &str, root_account_id: Uuid) {
    let account_id = create_account(url, root_account_id, "Test").await;
    tracing::info!("account_id = {}", account_id);

    let client = Client::new(url, account_id);

    let project_id = client.create_project("Test project").await.unwrap();
    tracing::info!("project_id = {}", project_id);

    let drawing_id = client
        .create_drawing(project_id, "Tree", 800, 600)
        .await
        .unwrap();
    tracing::info!("drawing_id = {}", drawing_id);

    let rectangle_id = client
        .create_object(
            drawing_id,
            json!({
                "color": { "red": 1.0, "green": 0.0, "blue": 0.0 },
                "opacity": 1.0,
            }),
            json!({
                "color": { "red": 0.0, "green": 0.0, "blue": 0.0 },
                "width": 4.0,
                "style": "Solid",
            }),
            json!({
                "Rectangle": {
                    "origin": { "x": 40, "y": 80 },
                    "size": { "width": 100, "height": 50 },
                }
            }),
        )
        .await
        .unwrap();

    client
        .create_object(
            drawing_id,
            json!({
                "color": { "red": 0.0, "green": 1.0, "blue": 0.0 },
                "opacity": 1.0,
            }),
            json!({
                "color": { "red": 0.0, "green": 0.75, "blue": 0.0 },
                "width": 4.0,
                "style": "Solid",
            }),
            json!({
                "Circle": {
                    "center": { "x": 240, "y": 160 },
                    "radius": 60,
                }
            }),
        )
        .await
        .unwrap();

    let triangle_id = client
        .create_object(
            drawing_id,
            json!({
                "color": { "red": 0.0, "green": 1.0, "blue": 1.0 },
                "opacity": 1.0,
            }),
            json!({
                "color": { "red": 0.0, "green": 0.0, "blue": 0.0 },
                "width": 4.0,
                "style": "Solid",
            }),
            json!({
                "Triangle": {
                    "p1": { "x": 80, "y": 160 },
                    "p2": { "x": 120, "y": 280 },
                    "p3": { "x": 180, "y": 220 },
                }
            }),
        )
        .await
        .unwrap();

    let svg = client.drawing_svg(drawing_id).await.unwrap();
    std::fs::write("output1.svg", svg.clone()).unwrap();
    tracing::info!("Wrote to {}", "output1.svg");
    assert_eq!(svg, OUTPUT1_EXPECTED);

    client.delete_object(rectangle_id).await.unwrap();
    client
        .update_object(
            triangle_id,
            Some(json!({
                "color": { "red": 1.0, "green": 1.0, "blue": 0.0 },
                "opacity": 1.0,
            })),
            Some(json!({
                "color": { "red": 0.0, "green": 0.0, "blue": 1.0 },
                "width": 4.0,
                "style": "Solid",
            })),
            Some(json!({
                "Triangle": {
                    "p1": { "x": 80, "y": 160 },
                    "p2": { "x": 60, "y": 280 },
                    "p3": { "x": 140, "y": 260 },
                }
            })),
        )
        .await
        .unwrap();

    let svg = client.drawing_svg(drawing_id).await.unwrap();
    std::fs::write("output2.svg", svg.clone()).unwrap();
    tracing::info!("Wrote to {}", "output2.svg");
    assert_eq!(svg, OUTPUT2_EXPECTED);
}

#[instrument(skip_all)]
async fn test_permissions(url: &str, root_account_id: Uuid) {
    let account1 = create_account(url, root_account_id, "Account 1").await;
    let account2 = create_account(url, root_account_id, "Account 2").await;
    let account3 = create_account(url, root_account_id, "Account 3").await;
    let account4 = create_account(url, root_account_id, "Account 3").await;

    let client1 = Client::new(url, account1);

    let project1 = client1.create_project("Project 1").await.unwrap();
    let project2 = client1.create_project("Project 2").await.unwrap();
    let project3 = client1.create_project("Project 3").await.unwrap();

    client1
        .set_project_permission(project1, account2, Some("write"))
        .await
        .unwrap();
    client1
        .set_project_permission(project1, account3, Some("read"))
        .await
        .unwrap();

    client1
        .set_project_permission(project2, account2, Some("read"))
        .await
        .unwrap();
    client1
        .set_project_permission(project3, account2, Some("admin"))
        .await
        .unwrap();
    client1
        .set_project_permission(project3, account4, Some("read"))
        .await
        .unwrap();

    let mut result = graphql(
        url,
        account1,
        r#"
            query($id: String!) {
                project(id: $id) {
                    name
                    permissions {
                        account {
                            id
                            name
                        }
                        permission
                    }
                }
            }
        "#,
        Some(json!({ "id": project1 })),
    )
    .await
    .unwrap();

    result["project"]["permissions"]
        .as_array_mut()
        .unwrap()
        .sort_by(|a, b| {
            a["account"]["name"]
                .as_str()
                .unwrap()
                .cmp(b["account"]["name"].as_str().unwrap())
        });

    assert_eq!(
        result,
        json!({
            "project": {
                "name": "Project 1",
                "permissions": [
                {
                    "account": { "id": account1, "name": "Account 1" },
                    "permission": "admin"
                },
                {
                    "account": { "id": account2, "name": "Account 2" },
                    "permission": "write"
                },
                {
                    "account": { "id": account3, "name": "Account 3" },
                    "permission": "read"
                }
                ]
            }
        })
    );

    let mut result = graphql(
        url,
        account2,
        r#"
            query($id: String!) {
                account(id: $id) {
                    permissions {
                        project {
                            id
                            name
                        }
                        permission
                    }
                }
            }
        "#,
        Some(json!({ "id": account2 })),
    )
    .await
    .unwrap();

    result["account"]["permissions"]
        .as_array_mut()
        .unwrap()
        .sort_by(|a, b| {
            a["project"]["name"]
                .as_str()
                .unwrap()
                .cmp(b["project"]["name"].as_str().unwrap())
        });

    assert_eq!(
        result,
        json!({
            "account": {
                "permissions": [
                {
                    "project": { "id": project1, "name": "Project 1" },
                    "permission": "write"
                },
                {
                    "project": { "id": project2, "name": "Project 2" },
                    "permission": "read"
                },
                {
                    "project": { "id": project3, "name": "Project 3" },
                    "permission": "admin"
                }
                ]
            }
        })
    );

    let account1_projects = Client::new(url, account1).list_projects().await.unwrap();
    assert_eq!(
        account1_projects,
        json!([
            { "id": project1, "name": "Project 1", "permission": "admin" },
            { "id": project2, "name": "Project 2", "permission": "admin" },
            { "id": project3, "name": "Project 3", "permission": "admin" }
        ])
    );

    let account2_projects = Client::new(url, account2).list_projects().await.unwrap();
    assert_eq!(
        account2_projects,
        json!([
            { "id": project1, "name": "Project 1", "permission": "write" },
            { "id": project2, "name": "Project 2", "permission": "read" },
            { "id": project3, "name": "Project 3", "permission": "admin" }
        ])
    );

    let account3_projects = Client::new(url, account3).list_projects().await.unwrap();
    assert_eq!(
        account3_projects,
        json!([
            { "id": project1, "name": "Project 1", "permission": "read" },
        ])
    );

    let account4_projects = Client::new(url, account4).list_projects().await.unwrap();
    assert_eq!(
        account4_projects,
        json!([
            { "id": project3, "name": "Project 3", "permission": "read" }
        ])
    );
}

#[instrument(skip_all)]
async fn test_boolean_operators(url: &str, root_account_id: Uuid) {
    let account_id = create_account(url, root_account_id, "Account 1").await;
    let client = Client::new(url, account_id);
    let project_id = client.create_project("Project 1").await.unwrap();

    for (name, width, height) in [
        ("Drawing 0", 300, 100),
        ("Drawing 1", 400, 110),
        ("Drawing 2", 500, 120),
        ("Drawing 3", 600, 130),
        ("Drawing 4", 700, 140),
        ("Drawing 5", 300, 150),
        ("Drawing 6", 400, 160),
        ("Drawing 7", 500, 170),
        ("Drawing 8", 600, 180),
        ("Drawing 9", 700, 190),
    ] {
        client
            .create_drawing(project_id, name, width, height)
            .await
            .unwrap();
    }

    // not
    let query = r#"{ drawings(order_by: { name: ASC }, filters: {
        not: { width: { gt: 400 } }
    }) { nodes { name width height } } }"#;
    let result = graphql(url, account_id, query, None).await.unwrap();
    let expected1 = json!({
        "drawings": {
            "nodes": [
                { "name": "Drawing 0", "width": 300, "height": 100},
                { "name": "Drawing 1", "width": 400, "height": 110},
                { "name": "Drawing 5", "width": 300, "height": 150},
                { "name": "Drawing 6", "width": 400, "height": 160}
            ]
        }
    });
    assert_eq!(result, expected1);

    // and
    let expected2 = json!({
        "drawings": {
            "nodes": [
                { "name": "Drawing 2", "width": 500, "height": 120 },
                { "name": "Drawing 3", "width": 600, "height": 130 },
                { "name": "Drawing 4", "width": 700, "height": 140 },
                { "name": "Drawing 7", "width": 500, "height": 170 }
            ]
        }
    });

    let query = r#"{ drawings(order_by: { name: ASC }, filters: {
        and: [
            { width: { gte: 500 } }
            { height: { lte: 170 } }
        ]
    }) { nodes { name width height } } }"#;
    let result = graphql(url, account_id, query, None).await.unwrap();
    assert_eq!(result, expected2);

    // and + not
    // Same as above but with conditions inverted
    let query = r#"{ drawings(order_by: { name: ASC }, filters: {
        and: [
            { not: { width: { lt: 500 } } }
            { not: { height: { gt: 170 } } }
        ]
    }) { nodes { name width height } } }"#;
    let result = graphql(url, account_id, query, None).await.unwrap();
    assert_eq!(result, expected2);

    let expected3 = json!({
        "drawings": {
            "nodes": [
                { "name": "Drawing 0", "width": 300, "height": 100 },
                { "name": "Drawing 1", "width": 400, "height": 110 },
                { "name": "Drawing 4", "width": 700, "height": 140 },
                { "name": "Drawing 9", "width": 700, "height": 190 }
            ]
        }
    });

    // or
    let query = r#"{ drawings(order_by: { name: ASC }, filters: {
        or: [
            { width: { gte: 700 } }
            { height: { lte: 110 } }
        ]
    }) { nodes { name width height } } }"#;
    let result = graphql(url, account_id, query, None).await.unwrap();
    assert_eq!(result, expected3);

    // or + not
    // Same as above but with conditions inverted
    let query = r#"{ drawings(order_by: { name: ASC }, filters: {
        or: [
            { not: { width: { lt: 700 } } }
            { not: { height: { gt: 110 } } }
        ]
    }) { nodes { name width height } } }"#;
    let result = graphql(url, account_id, query, None).await.unwrap();
    assert_eq!(result, expected3);
}

pub async fn create_account(url: &str, root_account_id: Uuid, name: &str) -> Uuid {
    let result = graphql(
        url,
        root_account_id,
        r#"
            mutation($name: String!, $email: String!) {
                create_account(name: $name, email: $email) {
                    id
                }
            }
        "#,
        Some(json!({
            "name": name,
            "email": "user@example.com",
        })),
    )
    .await
    .unwrap();

    serde_json::from_value::<Uuid>(result["create_account"]["id"].clone()).unwrap()
}

async fn wait_for_server(url: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let interval = 500;
    let max_retries = 20;
    let mut retries = 0;
    loop {
        match graphql(url, unauthenticated(), "mutation { _ping }", None).await {
            Ok(_) => {
                break;
            }
            Err(e) => {
                if retries == max_retries {
                    tracing::error!("Connection failed after {} attempts: {}", retries, e);
                    return Err(Box::new(e));
                } else {
                    tracing::info!("Server not yet ready; retrying in {}ms", interval);
                    tokio::time::sleep(Duration::from_millis(interval)).await;
                    retries += 1;
                }
            }
        }
    }
    Ok(())
}
