use clap::Parser;
use dotenv::dotenv;
use serde_json::json;
use uuid::Uuid;

use sea_draw::client::{client::Client, graphql::graphql};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Binding address at which the API will be available.
    #[clap(short, long, default_value = "127.0.0.1", env = "ADDRESS")]
    address: String,

    /// Port at which the API will be available.
    #[clap(short, long, default_value_t = 3333, env = "PORT")]
    port: u16,

    #[clap(long, env = "ROOT_ACCOUNT_ID")]
    root_account_id: Uuid,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let args = Args::parse();
    let url = &format!("http://{}:{}", args.address, args.port);

    println!("Client test: url = {}", url);
    println!("root token = {}", args.root_account_id);

    let account_id = create_account(url, args.root_account_id).await;
    println!("account_id = {}", account_id);

    let client = Client::new(url.clone(), account_id);

    let project_id = client.create_project("Test project").await.unwrap();
    println!("project_id = {}", project_id);

    let drawing_id = client
        .create_drawing(project_id, "Tree", 800, 600)
        .await
        .unwrap();
    println!("drawing_id = {}", drawing_id);

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
    let output_filename = "output1.svg";
    std::fs::write(output_filename, svg).unwrap();
    println!("Wrote to {}", output_filename);

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
    let output_filename = "output2.svg";
    std::fs::write(output_filename, svg).unwrap();
    println!("Wrote to {}", output_filename);

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
