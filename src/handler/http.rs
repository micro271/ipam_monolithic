use tera::{Tera, Context};
use uuid::Uuid;
use std::{collections::HashMap, sync::LazyLock};
use axum::{extract::{Path, Request, State}, response::{Html, IntoResponse}};
use tokio::sync::Mutex;

use crate::{database::repository::Repository, models::{network::Network, device::Device}};

use super::RepositoryType;
static TEMPLATES: LazyLock<Mutex<Tera>> = LazyLock::new(|| {
    Mutex::new(Tera::new("templates/**/*").expect("template dir doesn't found"))
});


pub async fn login() -> impl IntoResponse {
    let tera = TEMPLATES.lock().await;

    let context = Context::new();

    Html(tera.render("login.html", &context).unwrap()).into_response()
}


pub async fn fallback(req: Request) -> impl IntoResponse {
    let tera = TEMPLATES.lock().await;

    let mut context = Context::new();
    context.insert("path", &req.uri().to_string());

    Html(tera.render("fallback.html", &context).unwrap()).into_response()
}

pub async fn http_view_network(State(state): State<RepositoryType>) -> impl IntoResponse {
    let state = state.lock().await;

    let networks = state.get::<Network>(None).await.unwrap();
    
    let mut cont = Context::new();
    cont.insert("block", "network");
    cont.insert("networks", &networks);
    cont.insert("title", "Networks");

    let tera = TEMPLATES.lock().await;

    Html(tera.render("index.html", &cont).unwrap()).into_response()
}

pub async fn http_view_devices(State(state): State<RepositoryType>, Path(network_id): Path<Uuid>) -> impl IntoResponse {
    let state = state.lock().await;

    let network = state.get::<Network>(Some(HashMap::from([("id",network_id.into())]))).await.unwrap();

    let devices = state.get::<Device>(Some(HashMap::from([("network_id", network_id.into())]))).await.unwrap();
    
    let mut con = Context::new();
    con.insert("block", "device");
    con.insert("network",&network.first().map(|x| x.network));
    con.insert("devices", &devices);
    let tera = TEMPLATES.lock().await;
    Html(tera.render("index.html", &con).unwrap()).into_response()
}

