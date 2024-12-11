use std::collections::HashMap;

use super::{
    query_params::{ParamPKService, ParamPKServiceGet},
    utils::TypeTable,
    RepositoryType, ResponseError, Uri,
};
use crate::{
    database::repository::{QueryResult, Repository},
    models::service::{Service, ServiceUpdate},
};
use axum::{
    extract::{Query, State},
    Json,
};
use libipam::response_error::Builder;

pub async fn create(
    State(state): State<RepositoryType>,
    Json(service): Json<Service>,
) -> Result<QueryResult<Service>, ResponseError> {
    let state = state.lock().await;

    Ok(state.insert::<Service>(vec![service]).await?)
}

pub async fn update(
    State(state): State<RepositoryType>,
    uri: Uri,
    Query(ParamPKService {
        port,
        ip,
        network_id,
    }): Query<ParamPKService>,
    Json(updater): Json<ServiceUpdate>,
) -> Result<QueryResult<Service>, ResponseError> {
    let state = state.lock().await;

    Ok(state
        .update(
            updater,
            Some(HashMap::from([
                ("port", port.into()),
                ("ip", ip.into()),
                ("network_id", network_id.into()),
            ])),
        )
        .await
        .map_err(|x| Into::<Builder>::into(ResponseError::from(x)).instance(uri.to_string()))?)
}

pub async fn delete(
    State(state): State<RepositoryType>,
    Query(ParamPKService {
        port,
        ip,
        network_id,
    }): Query<ParamPKService>,
) -> Result<QueryResult<Service>, ResponseError> {
    let state = state.lock().await;

    Ok(state
        .delete(Some(HashMap::from([
            ("port", port.into()),
            ("ip", ip.into()),
            ("network_id", network_id.into()),
        ])))
        .await?)
}

pub async fn get(
    State(state): State<RepositoryType>,
    Query(ParamPKServiceGet {
        ip,
        network_id,
        port,
    }): Query<ParamPKServiceGet>,
) -> Result<QueryResult<Service>, ResponseError> {
    let state = state.lock().await;
    let mut condition: HashMap<_, TypeTable> =
        HashMap::from([("ip", ip.into()), ("network_id", network_id.into())]);
    if let Some(port) = port {
        condition.insert("port", port.into());
    }
    Ok(state.get(Some(condition)).await?.into())
}
