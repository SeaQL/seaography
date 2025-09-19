use async_graphql::http::{GraphQLPlaygroundConfig, graphiql_source, playground_source};
use axum::{Extension, Router, response, routing};
use clap::Parser;
use dotenv::dotenv;
use sea_orm::{
    Database, ModelTrait,
    entity::{EntityTrait},
};
use std::net::{IpAddr, SocketAddr};
use tokio::net::TcpListener;

use sea_draw::{
    backend::Backend,
    entities::{Account, Drawing, Object, Project},
    schema::queries_and_mutations,
    types::{Circle, Fill, Point, Shape, Stroke},
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Binding address at which the API will be available.
    #[clap(short, long, default_value = "127.0.0.1", env = "ADDRESS")]
    address: String,

    /// Port at which the API will be available.
    #[clap(short, long, default_value_t = 3333, env = "PORT")]
    port: u16,

    #[clap(long, env = "DATABASE_URL")]
    database_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let args = Args::parse();

    println!("================================================================================");

    let database = Database::connect(args.database_url)
        .await
        .expect("Fail to initialize database connection");
    println!("Connected to database");

    // let statement = Statement::from_string(database.get_database_backend(), SQLITE_SCHEMA);
    // database.execute(statement).await.unwrap();

    let backend = Backend::new(database);

    // let project_model = entities::project::Model {
    //     id: Uuid::new_v4(),
    //     last_update: Utc::now(),
    //     name: "Test".to_string(),
    // };

    // let insert = entities::project::Entity::insert(project_model.into_active_model().reset_all());
    // insert.exec(&database).await.unwrap();

    let project1 = backend.create_project("Project 1").await.unwrap();
    println!("project1 = {}", project1.id);
    let project2 = backend.create_project("Project 2").await.unwrap();
    println!("project2 = {}", project2.id);

    let drawing1 = backend
        .create_drawing(project1.id, "House", 1024, 768)
        .await
        .unwrap();
    println!("drawing1 = {}", drawing1.id);
    let drawing2 = backend
        .create_drawing(project1.id, "Car", 640, 480)
        .await
        .unwrap();
    println!("drawing2 = {}", drawing2.id);

    let object1 = backend
        .create_object(
            project1.id,
            drawing1.id,
            Fill::default(),
            Stroke::default(),
            Shape::Circle(Circle {
                center: Point { x: 0.0, y: 0.0 },
                radius: 2.0,
            }),
        )
        .await
        .unwrap();
    println!("object1 = {}", object1.id);
    let object2 = backend
        .create_object(
            project1.id,
            drawing1.id,
            Fill::default(),
            Stroke::default(),
            Shape::Circle(Circle {
                center: Point { x: 0.0, y: 0.0 },
                radius: 2.0,
            }),
        )
        .await
        .unwrap();
    println!("object2 = {}", object2.id);

    let all_projects: Vec<Project> = <Project as ModelTrait>::Entity::find()
        .all(&backend.db)
        .await
        .unwrap();
    println!("Found {} projects", all_projects.len());
    println!("{:#?}", all_projects);

    let all_drawings: Vec<Drawing> = <Drawing as ModelTrait>::Entity::find()
        .all(&backend.db)
        .await
        .unwrap();
    println!("Found {} drawings", all_drawings.len());
    println!("{:#?}", all_drawings);

    let all_objects: Vec<Object> = <Object as ModelTrait>::Entity::find()
        .all(&backend.db)
        .await
        .unwrap();
    println!("Found {} objects", all_objects.len());
    println!("{:#?}", all_objects);

    let account1 = backend
        .create_account("Account 1", "one@example.com")
        .await
        .unwrap();
    println!("account1 = {}", account1.id);
    let account2 = backend
        .create_account("Account 2", "two@example.com")
        .await
        .unwrap();
    println!("account2 = {}", account2.id);

    let all_accounts: Vec<Account> = <Account as ModelTrait>::Entity::find()
        .all(&backend.db)
        .await
        .unwrap();
    println!("Found {} accounts", all_accounts.len());
    println!("{:#?}", all_accounts);

    println!("Starting GraphQL server");
    let schema = sea_draw::schema::schema(backend).unwrap();

    let graphql_endpoint_url = format!("http://{}:{}", args.address, args.port);
    let subscription_endpoint = format!("http://{}:{}/ws", args.address, args.port);

    let app = Router::new()
        .route(
            "/",
            routing::get(
                response::Html(
                    playground_source(
                        GraphQLPlaygroundConfig::new(&args.address),
                    )
                )
            )
            .post(queries_and_mutations),
        )
        .route(
            "/graphiql",
            routing::get(
                response::Html(
                    graphiql_source(
                        &graphql_endpoint_url,
                        Some(&subscription_endpoint),
                    )
                )
            )
        )
        // .route("/ws", routing::get(graphql_ws_handler))
        .layer(Extension(schema))
        // .layer(Extension(provider))
        // .layer(DefaultBodyLimit::max(1024 * 1024 * 1024))
        // .layer(RequestBodyLimitLayer::new(1024 * 1024 * 1024))
        // .nest("/public", public::get_routes(Arc::new(public_config)))
        // .layer(cors_layer)
        ;
    println!(
        "Visit GraphQL Playground at http://{}:{} or http://{}:{}/graphiql",
        args.address, args.port, args.address, args.port
    );
    let socketaddr: SocketAddr = (
        args.address.parse::<IpAddr>().expect("invalid ip address"),
        args.port,
    )
        .into();

    let serve_handle = axum::serve(TcpListener::bind(&socketaddr).await.unwrap(), app);

    tokio::select!(
        // _ =  multiplexer_handle => {
        //     tracing::error!("Multiplexer task completed")
        // }
        // _ = multiplexer_gc_handle => {
        //     tracing::error!("Multiplexer gc task completed")
        // }
        // _ = postgres_listener_handle => {
        //     tracing::error!("Postgres listener task completed")
        // }
        _ = serve_handle => {
            tracing::error!("Serve handle completed")
        }
    );

    Ok(())
}
