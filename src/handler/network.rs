use super::*;
use crate::models::network::*;
use libipam::response_error::ResponseError;
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
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ResponseError> {
    let state = state.lock().await;
    let condition = HashMap::from([("id", id.into())]);

    let network = state.get::<Network>(Some(condition)).await?;

    Ok(Json(json!({
        "device": network.first()
    })))
}

pub async fn update(
    State(state): State<RepositoryType>,
    Extension(claim): Extension<Claims>,
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
    Ok(state
        .delete::<crate::models::device::Device>(Some(HashMap::from([("network_id", id.into())])))
        .await?)
}

pub async fn get_all(
    State(state): State<RepositoryType>,
) -> Result<impl IntoResponse, ResponseError> {
    let state = state.lock().await;

    let networks = state.get::<Network>(None).await?;

    Ok(Json(json!({
        "networks": networks
    })))
}

pub async fn delete(
    State(state): State<RepositoryType>,
    Extension(claim): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ResponseError> {
    if claim.role != Role::Admin {
        return Err(ResponseError::builder()
            .status(StatusCode::UNAUTHORIZED)
            .build());
    }

    let state = state.lock().await;

    Ok(state
        .delete::<Network>(Some(HashMap::from([("id", id.into())])))
        .await?)
}
