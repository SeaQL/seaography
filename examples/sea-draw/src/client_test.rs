use serde_json::json;
use std::time::Duration;
use tracing::instrument;
use uuid::Uuid;

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
    let url = &format!("http://{}:{}", address, port);
    wait_for_server(url).await?;

    tracing::info!("Client test: url = {}", url);
    tracing::info!("root token = {}", root_account_id);

    let account_id = create_account(url, root_account_id).await;
    tracing::info!("account_id = {}", account_id);

    let client = Client::new(url.clone(), account_id);

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

    tracing::info!("All tests completed successfully!");

    Ok(())
}

pub async fn create_account(url: &str, root_account_id: Uuid) -> Uuid {
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
            "name": "Test account",
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
