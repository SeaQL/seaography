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

    #[clap(long, env = "ROOT_TOKEN")]
    root_token: Uuid,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let args = Args::parse();
    let url = &format!("http://{}:{}", args.address, args.port);

    println!("Client test: url = {}", url);
    println!("root token = {}", args.root_token);

    // let result = graphql(
    //     url,
    //     args.root_token,
    //     r#"
    //         mutation($name: String!, $email: String!) {
    //             create_account(name: $name, email: $email) {
    //                 id
    //             }
    //         }
    //     "#,
    //     Some(json!({
    //         "name": "Test account",
    //         "email": "user@example.com",
    //     })),
    // ).await.unwrap();
    // println!("result = {}", serde_json::to_string_pretty(&result).unwrap());

    // let account_id = serde_json::from_value::<Uuid>(result["create_account"]["id"].clone()).unwrap();
    // println!("account_id = {}", account_id);

    let account_id = create_account(url, args.root_token).await;
    println!("account_id = {}", account_id);

    let client = Client::new(url.clone(), account_id);

    let project_id = client.create_project("Test project").await.unwrap();
    println!("project_id = {}", project_id);

    let drawing_id = client
        .create_drawing(project_id, "Tree", 1024, 768)
        .await
        .unwrap();
    println!("drawing_id = {}", drawing_id);

    let object_id = client
        .create_object(
            drawing_id,
            json!({
                "color": { "red": 0.0, "green": 0.0, "blue": 0.0 },
                "opacity": 1.0,
            }),
            json!({
                "color": { "red": 1.0, "green": 0.0, "blue": 0.0 },
                "width": 1.0,
                "style": "Solid",
            }),
            json!({
                "Rectangle": {
                    "origin": { "x": 4.0, "y": 7.0 },
                    "size": { "width": 10.0, "height": 5.0 },
                }
            }),
        )
        .await
        .unwrap();
    println!("object_id = {}", object_id);

    Ok(())
}

pub async fn create_account(url: &str, root_token: Uuid) -> Uuid {
    let result = graphql(
        url,
        root_token,
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
    // println!("result = {}", serde_json::to_string_pretty(&result).unwrap());

    serde_json::from_value::<Uuid>(result["create_account"]["id"].clone()).unwrap()
}
