mod database;
mod handler;
mod models;
mod user;

use axum::{
    http::Response,
    middleware,
    routing::{delete, get, post},
    serve, Router,
};
use database::SqliteRepository;
use dotenv::dotenv;
use handler::*;
use std::{env, path};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let ip = env::var("IP_ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("3000".to_string());

    let lst = tokio::net::TcpListener::bind(format!("{}:{}", ip, port)).await?;

    let db_name = env::var("DB_NAME").unwrap_or("data".to_string());

    if db_name.contains("/") {
        return Err("Database file can't belong to a directory".into());
    }
    let path_db = format!("./{db_name}.db");

    let flag_create_tables = !std::path::Path::new(&path_db).exists();

    if flag_create_tables {
        std::fs::File::create(std::path::Path::new(&path_db)).expect("Don't can't create the db file");
    }

    let db = SqliteRepository::new(&format!("sqlite://{db_name}.db")).await?;

    if flag_create_tables {
        let query = include_str!("../initdb.sql");
        
        sqlx::query(query).execute(&*db).await?;
    }

    user::create_default_user(&db).await?;


    let db = Arc::new(Mutex::new(db));
    let network = Router::new()
        .route("/create", post(network::create))
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
                .patch(device::update)
                .post(device::create_all_devices),
        ) // create, update and get all devices
        .route("/delete", delete(device::delete))
        .route("/one", get(device::get_one)); //get one device

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
        .body("<h1>Bienvenido</h1>".to_string())
        .unwrap_or_default()
}
