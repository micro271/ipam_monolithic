mod database;
mod handler;
mod models;
mod services;
mod trace_layer;

use axum::{
    middleware,
    response::{IntoResponse, Redirect},
    routing::{delete, get, patch, post},
    serve, Router,
};
use database::SqliteRepository;
use handler::{services as svcs, *};
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
        .route("/clean/:id", delete(network::clean))
        .route("/", post(network::create).get(network::get_all))
        .route(
            "/subnet",
            post(network::create_network_child).get(network::get_all_with_father),
        )
        .route(
            "/:id",
            get(network::get_one)
                .patch(network::update)
                .delete(network::delete),
        );

    let device = Router::new()
        .route(
            "/",
            post(device::create)
                .delete(device::delete)
                .patch(device::update)
                .get(device::get),
        )
        .route("/:network_id", post(device::create_all_devices)) // create, update and get all devices
        .route("/ping", patch(device::ping))
        .route("/reserve", patch(device::reserve));

    let user = Router::new().route("/", post(auth::create));

    let service = Router::new().route(
        "/",
        post(service::create)
            .patch(service::update)
            .delete(service::delete)
            .get(service::get),
    );

    let services = Router::new().route("/", post(svcs::create)).route(
        "/:id",
        patch(svcs::update).get(svcs::get).delete(svcs::delete),
    );

    let api = Router::new()
        .nest("/service", service)
        .nest("/services", services)
        .nest("/network", network)
        .nest("/device", device)
        .nest("/user", user);

    let web = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/", get(http::http_view_network))
        .route("/offices", get(http::offices))
        .route(
            "/favicon.ico",
            get(|| async { Redirect::to("/static/favicon.ico").into_response() }),
        )
        .route("/:network_id", get(http::http_view_devices));

    let app = Router::new()
        .nest("/", web)
        .nest("/api/v1", api)
        .layer(middleware::from_fn(auth::verify_token))
        .route("/login", post(auth::login).get(http::login))
        .with_state(db.clone())
        .fallback(http::fallback)
        .layer(
            tower::ServiceBuilder::new()
                .layer(CompressionLayer::new().br(true).gzip(true).deflate(true))
                .layer(trace_layer)
                .into_inner(),
        );

    serve(lst, app).await?;

    Ok(())
}
