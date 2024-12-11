use super::{office::Office, query_params::ParamPKServiceGet, utils::TypeTable};
use axum::{
    extract::{Path, Query, Request, State},
    response::{Html, IntoResponse},
    Extension,
};
use std::{collections::HashMap, sync::LazyLock};
use tera::{Context, Tera};
use tokio::sync::Mutex;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    database::repository::Repository,
    models::{
        device::Device,
        network::Network,
        service::{Service, Services},
        user::Role,
    },
    services::Claims,
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

pub async fn http_view_network(
    State(state): State<RepositoryType>,
    Extension(claim): Extension<Claims>,
) -> impl IntoResponse {
    let state = state.lock().await;

    let networks = state
        .get::<Network>(Some(HashMap::from([("father", TypeTable::Null)])))
        .await
        .unwrap_or_default();

    let mut cont = Context::new();
    cont.insert("block", "network");
    cont.insert("networks", &networks);
    cont.insert("role", &claim.role);
    cont.insert("user_id", &claim.sub);
    cont.insert("username", &claim.username);

    let tera = TEMPLATES.lock().await;
    Html(tera.render("index.html", &cont).unwrap()).into_response()
}
pub async fn http_view_devices(
    State(state): State<RepositoryType>,
    Extension(claim): Extension<Claims>,
    Path(network_id): Path<Uuid>,
) -> impl IntoResponse {
    let state = state.lock().await;

    let network = state
        .get::<Network>(Some(HashMap::from([("id", network_id.into())])))
        .await
        .unwrap();
    let network_chiled = state
        .get::<Network>(Some(HashMap::from([("father", network_id.into())])))
        .await
        .unwrap_or_default();

    let mut devices = state
        .get::<Device>(Some(HashMap::from([("network_id", network_id.into())])))
        .await
        .unwrap_or_default();

    if !devices.is_empty() {
        devices.sort_by_key(|x| x.ip);
    }

    if claim.role != Role::Admin {
        for dev in devices.iter_mut() {
            dev.credential = None;
        }
    }
    tracing::info!("Network: {:?}", network);
    let mut con = Context::new();
    con.insert("block", "device");
    con.insert("network", &network.first());
    con.insert("devices", &devices);
    con.insert("subnet", &network_chiled);
    con.insert("user_id", &claim.sub);
    con.insert("username", &claim.username);
    con.insert("role", &claim.role);
    con.insert("ipv4", &network.first().map(|x| x.network.addr().is_ipv4()));

    let tera = TEMPLATES.lock().await;
    Html(tera.render("index.html", &con).unwrap()).into_response()
}

pub async fn offices(
    State(state): State<RepositoryType>,
    Extension(claim): Extension<Claims>,
) -> impl IntoResponse {
    let mut cont = Context::new();
    cont.insert("block", "office");
    cont.insert("user_id", &claim.sub);
    cont.insert("role", &claim.role);
    cont.insert("username", &claim.username);

    let state = state.lock().await;
    let ofs = state.get::<Office>(None).await.unwrap_or_default();
    cont.insert("offices", &ofs);
    let tera = TEMPLATES.lock().await;
    Html(tera.render("index.html", &cont).unwrap()).into_response()
}

pub async fn services(
    State(state): State<RepositoryType>,
    Extension(claim): Extension<Claims>,
) -> impl IntoResponse {
    let state = state.lock().await;
    let svcs = state.get::<Services>(None).await.unwrap_or_default();
    let mut ctx = Context::new();
    ctx.insert("services", &svcs);
    ctx.insert("user_id", &claim.sub);
    ctx.insert("role", &claim.role);
    ctx.insert("username", &claim.username);

    let tera = TEMPLATES.lock().await;
    Html(tera.render("index.html", &ctx).unwrap()).into_response()
}

pub async fn service(
    State(state): State<RepositoryType>,
    Extension(claim): Extension<Claims>,
    Query(ParamPKServiceGet {
        port,
        ip,
        network_id,
    }): Query<ParamPKServiceGet>,
) -> impl IntoResponse {
    let state = state.lock().await;
    let mut ctx = Context::new();
    let mut condition: HashMap<_, TypeTable> =
        HashMap::from([("ip", ip.into()), ("network_id", network_id.into())]);
    if let Some(port) = port {
        condition.insert("port", port.into());
    }
    let service = state
        .get::<Service>(Some(condition))
        .await
        .unwrap_or_default();

    ctx.insert("services", &service);
    ctx.insert("user_id", &claim.sub);
    ctx.insert("role", &claim.role);
    ctx.insert("username", &claim.username);

    let tera = TEMPLATES.lock().await;

    Html(tera.render("index.html", &ctx).unwrap()).into_response()
}

pub(super) mod filter {
    pub fn truncate_with_ellipsis() {
        todo!()
    }
}
