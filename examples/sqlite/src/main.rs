use actix_web::{guard, web, web::Data, App, HttpRequest, HttpResponse, HttpServer, Result};
use async_graphql::{
    dynamic::*,
    http::{playground_source, GraphQLPlaygroundConfig},
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenv::dotenv;
use sea_orm::Database;
use seaography::{async_graphql, lazy_static};
use std::env;

lazy_static::lazy_static! {
    static ref URL: String = env::var("URL").unwrap_or("localhost:8000".into());
    static ref ENDPOINT: String = env::var("ENDPOINT").unwrap_or("/".into());
    static ref DATABASE_URL: String =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    static ref DEPTH_LIMIT: Option<usize> = env::var("DEPTH_LIMIT").map_or(None, |data| Some(
        data.parse().expect("DEPTH_LIMIT is not a number")
    ));
    static ref COMPLEXITY_LIMIT: Option<usize> = env::var("COMPLEXITY_LIMIT")
        .map_or(None, |data| {
            Some(data.parse().expect("COMPLEXITY_LIMIT is not a number"))
        });
}

async fn index(
    schema: web::Data<Schema>,
    http_req: HttpRequest,
    gql_req: GraphQLRequest,
) -> GraphQLResponse {
    let mut gql_req = gql_req.into_inner();

    let mut is_public = true;
    if let Some(auth_header) = http_req.headers().get("Authorization") {
        if let Ok(auth_value) = auth_header.to_str() {
            use hmac::{Hmac, Mac};
            use jwt::VerifyWithKey;
            use sha2::Sha256;
            use std::collections::BTreeMap;

            if let Ok(key) = Hmac::<Sha256>::new_from_slice(b"some-secret") {
                if let Ok(claims) = auth_value.verify_with_key(&key) {
                    let claims: BTreeMap<String, i32> = claims;
                    gql_req = gql_req.data(seaography::UserContext {
                        user_id: claims["user_id"],
                    });
                    is_public = false;
                }
            }
        }
    }
    if is_public {
        gql_req = gql_req.data(seaography::UserContext {
            user_id: 3,
        });
    }

    schema.execute(gql_req).await.into()
}

async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new(&*ENDPOINT))))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();
    let database = Database::connect(&*DATABASE_URL)
        .await
        .expect("Fail to initialize database connection");

    seaography_sqlite_example::rbac::setup(&database)
        .await
        .expect("RBAC setup failed");

    // Setup RBAC
    if let Ok(_) = database.load_rbac().await {
        // Load RBAC from database
    } else {
        // Load RBAC placeholder
        database.replace_rbac(sea_orm::rbac::RbacEngine::from_snapshot(sea_orm::rbac::RbacSnapshot::danger_unrestricted()));
    }

    let schema =
        seaography_sqlite_example::query_root::schema(database, *DEPTH_LIMIT, *COMPLEXITY_LIMIT)
            .unwrap();
    println!("Visit GraphQL Playground at http://{}", *URL);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .to(graphql_playground),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
