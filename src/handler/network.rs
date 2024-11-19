use super::*;
use crate::models::network::*;
use axum::http::Uri;
use libipam::response_error::{Builder, ResponseError};
pub async fn create(
    State(state): State<RepositoryType>,
    Extension(claim): Extension<Claims>,
    Json(netw): Json<models_data_entry::Network>,
) -> Result<impl IntoResponse, ResponseError> {
    if claim.role != Role::Admin {
        return Err(ResponseError::builder()
            .status(StatusCode::UNAUTHORIZED)
            .build());
    }

    let state = state.lock().await;
    tracing::info!("New network {:?}", netw);
    Ok(state.insert::<Network>(vec![netw.into()]).await?)
}

pub async fn get_one(
    State(state): State<RepositoryType>,
    uri: Uri,
    Path(id): Path<Uuid>,
) -> Result<QueryResult<Network>, ResponseError> {
    let state = state.lock().await;
    let condition = HashMap::from([("id", id.into())]);

    Ok(state
        .get::<Network>(Some(condition))
        .await
        .map_err(|x| {
            let bl: Builder = ResponseError::from(x).into();
            bl.instance(uri.to_string()).build()
        })?
        .into())
}

pub async fn update(
    State(state): State<RepositoryType>,
    Extension(claim): Extension<Claims>,
    uri: Uri,
    Path(id): Path<Uuid>,
    Json(network): Json<UpdateNetwork>,
) -> Result<impl IntoResponse, ResponseError> {
    if claim.role != Role::Admin {
        return Err(ResponseError::builder()
            .status(StatusCode::UNAUTHORIZED)
            .build());
    }

    let state = state.lock().await;

    // Now: Delete all devices that belong to the network
    // Soon: Update all devices
    //     * Only if the prefix of the new network is bigger than or smaller than the current network

    let _tmp = state
        .update::<Network, _>(network, Some(HashMap::from([("id", id.into())])))
        .await?;
    state
        .delete::<crate::models::device::Device>(Some(HashMap::from([("network_id", id.into())])))
        .await
        .map_err(|x| {
            let bl: Builder = ResponseError::from(x).into();
            bl.instance(uri.to_string()).build()
        })
}

pub async fn get_all(
    State(state): State<RepositoryType>,
    uri: Uri,
) -> Result<QueryResult<Network>, ResponseError> {
    let state = state.lock().await;

    state
        .get::<Network>(None)
        .await
        .map(QueryResult::from)
        .map_err(|x| {
            let tmp: Builder = ResponseError::from(x).into();
            tmp.instance(uri.to_string()).build()
        })
}

pub async fn delete(
    State(state): State<RepositoryType>,
    Extension(claim): Extension<Claims>,
    uri: Uri,
    Path(id): Path<Uuid>,
) -> Result<QueryResult<Network>, ResponseError> {
    if claim.role != Role::Admin {
        return Err(ResponseError::builder()
            .status(StatusCode::UNAUTHORIZED)
            .build());
    }

    let state = state.lock().await;

    state
        .delete::<Network>(Some(HashMap::from([("id", id.into())])))
        .await
        .map_err(|x| {
            let tmp: Builder = ResponseError::from(x).into();
            tmp.instance(uri.to_string()).build()
        })
}
