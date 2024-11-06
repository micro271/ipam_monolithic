mod database;
mod handler;
mod models;
mod services;

use axum::{
    http::Response,
    middleware,
    routing::{delete, get, post, put},
    serve, Router,
};
use database::SqliteRepository;
use dotenv::dotenv;
use handler::*;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let ip = env::var("IP_ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("3000".to_string());

    let lst = tokio::net::TcpListener::bind(format!("{}:{}", ip, port)).await?;

    let db_name = env::var("DB_NAME").unwrap_or("./data.sqlite".to_string());

    let db = Arc::new(Mutex::new(SqliteRepository::new(&db_name).await?));
    let network = Router::new()
        .route("/create", put(network::create))
        .route("/all", get(network::get_all)) // crate, update and get (all) networks
        .route(
            "/:id",
            get(network::get_one)
                .patch(network::update)
                .delete(network::delete),
        ); // get one network

    let device = Router::new()
        .route("/create", post(device::create))
        .route(
            "/all/:network_id",
            get(device::get_all)
                .put(device::create_all_devices),
        ) // create, update and get all devices
        .route("/delete", delete(device::delete))
        .route("/one", get(device::get_one).patch(device::update)); //get one device

    let user = Router::new().route("/", post(auth::create));

    let app = Router::new()
        .route("/", get(hello_world))
        .nest("/network", network)
        .nest("/device", device)
        .nest("/user", user)
        .layer(middleware::from_fn(auth::verify_token))
        .route("/login", post(auth::login))
        .with_state(db.clone());

    serve(lst, app).await?;

    Ok(())
}

async fn hello_world() -> Response<String> {
    Response::builder()
        .status(200)
        .header("Content-Type", "text/html")
        .body("<h1>Welcome</h1>".to_string())
        .unwrap_or_default()
}
