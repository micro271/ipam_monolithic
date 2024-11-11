mod database;
mod handler;
mod models;
mod services;
mod trace_layer;


use axum::{
    middleware, response::{IntoResponse, Redirect}, routing::{delete, get, post, put}, serve, Router
};
use database::SqliteRepository;
use handler::*;
use std::{env, sync::Arc};
use tokio::sync::Mutex;
use tower_http::{compression::CompressionLayer, services::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let ip = env::var("IP_ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("3000".to_string());

    let lst = tokio::net::TcpListener::bind(format!("{}:{}", ip, port)).await?;

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace_layer::make_span)
        .on_request(trace_layer::on_request)
        .on_response(trace_layer::on_response);

    tracing::info!("Listening: {}:{}", ip, port);

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

    let user = Router::new().route("/", put(auth::create));

    let api = Router::new()
        .nest("/network", network)
        .nest("/device", device)
        .nest("/user", user);

    let web = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/", get(http::http_view_network))
        .route("/devices/:network_id", get(http::http_view_devices))
        .route("/favicon.ico", get(|| async {Redirect::to("/static/favicon.ico").into_response()}));

    let app = Router::new()
        .nest("/", web)
        .nest("/api", api)
        .layer(middleware::from_fn(auth::verify_token))
        .route("/login", post(auth::login).get(http::login))
        .with_state(db.clone())
        .fallback(http::fallback)
        .layer(
            tower::ServiceBuilder::new()
                .layer(CompressionLayer::new().br(true).gzip(true).deflate(true))
                .layer(trace_layer)
                .into_inner()
        );

    serve(lst, app).await?;

    Ok(())
}
