use axum::{
    extract::{Path, Request, State},
    response::{Html, IntoResponse}, Extension,
};
use tracing::instrument;
use std::{collections::HashMap, sync::LazyLock};
use tera::{Context, Tera};
use tokio::sync::Mutex;
use uuid::Uuid;
use super::{office::Office, Role};

use crate::{
    database::repository::Repository,
    models::{device::Device, network::Network},
};

use super::RepositoryType;
static TEMPLATES: LazyLock<Mutex<Tera>> =
    LazyLock::new(|| Mutex::new(Tera::new("templates/**/*").expect("template dir doesn't found")));

pub async fn login() -> impl IntoResponse {
    let tera = TEMPLATES.lock().await;

    let context = Context::new();

    Html(tera.render("login.html", &context).unwrap()).into_response()
}

#[instrument]
pub async fn fallback(req: Request) -> impl IntoResponse {
    let tera = TEMPLATES.lock().await;

    let mut context = Context::new();
    context.insert("path", &req.uri().to_string());
    tracing::error!("HAA");
    Html(tera.render("fallback.html", &context).unwrap()).into_response()
}

#[instrument]
pub async fn http_view_network(State(state): State<RepositoryType>, Extension(role): Extension<Role>) -> impl IntoResponse {
    let state = state.lock().await;

    let networks = state.get::<Network>(None).await.unwrap();

    let mut cont = Context::new();
    cont.insert("block", "network");
    cont.insert("networks", &networks);
    cont.insert("title", "Networks");
    cont.insert("role", &role);

    let tera = TEMPLATES.lock().await;

    Html(tera.render("index.html", &cont).unwrap()).into_response()
}

pub async fn http_view_devices(
    State(state): State<RepositoryType>,
    Extension(role): Extension<Role>,
    Path(network_id): Path<Uuid>,
) -> impl IntoResponse {
    let state = state.lock().await;

    let network = state
        .get::<Network>(Some(HashMap::from([("id", network_id.into())])))
        .await
        .unwrap_or_default();

    let devices = state
        .get::<Device>(Some(HashMap::from([("network_id", network_id.into())])))
        .await
        .unwrap_or_default();

    let mut con = Context::new();
    con.insert("block", "device");
    con.insert("network", &network.first());
    con.insert("devices", &devices);
    con.insert("role", &role);
    con.insert("ipv4", &network.first().map(|x|x.network.addr().is_ipv4()));

    let tera = TEMPLATES.lock().await;
    Html(tera.render("index.html", &con).unwrap()).into_response()
}

pub async fn offices(State(state): State<RepositoryType>, Extension(role): Extension<Role>) -> impl IntoResponse{
    let mut cont = Context::new();
    cont.insert("block", "office");
    cont.insert("role", &role);
    
    let state = state.lock().await;
    let ofs = state.get::<Office>(None).await.unwrap_or_default();
    cont.insert("offices", &ofs);
    let tera = TEMPLATES.lock().await;
    Html(tera.render("index.html", &cont).unwrap()).into_response()
}