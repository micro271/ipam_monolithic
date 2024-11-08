mod database;
mod handler;
mod models;
mod services;


use axum::{
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
use tower_http::services::ServeDir;

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
            get(device::get_all).put(device::create_all_devices),
        ) // create, update and get all devices
        .route("/delete", delete(device::delete))
        .route("/one", get(device::get_one).patch(device::update)); //get one device

    let login = Router::new().route("/", post(auth::create));
    
    let api = Router::new()
        .nest("/network", network)
        .nest("/device", device)
        .nest("/login", login);

    let web = Router::new()
        .nest_service("/static", ServeDir::new("templates/static"))
        .route("/", get(http::http_view_network))
        .route("/devices/:network_id", get(http::http_view_devices));

    let app = Router::new()
        .nest("/api", api)
        .layer(middleware::from_fn(auth::verify_token))
        .nest("/", web)
        .route("/login", post(auth::login).get(http::login))
        .with_state(db.clone())
        .fallback(http::fallback);

    serve(lst, app).await?;

    Ok(())
}
