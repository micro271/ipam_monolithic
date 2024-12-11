use response_error::Builder;

use super::*;
use crate::models::service::{Services, ServicesUpdate};

pub async fn create(
    State(state): State<RepositoryType>,
    Json(service): Json<Services>,
) -> Result<QueryResult<Services>, ResponseError> {
    let state = state.lock().await;

    Ok(state.insert::<Services>(vec![service]).await?)
}

pub async fn update(
    State(state): State<RepositoryType>,
    uri: Uri,
    Path(id): Path<uuid::Uuid>,
    Json(updater): Json<ServicesUpdate>,
) -> Result<QueryResult<Services>, ResponseError> {
    let state = state.lock().await;

    Ok(state
        .update(updater, Some(HashMap::from([("id", id.into())])))
        .await
        .map_err(|x| Into::<Builder>::into(ResponseError::from(x)).instance(uri.to_string()))?)
}

pub async fn delete(
    State(state): State<RepositoryType>,
    Path(id): Path<uuid::Uuid>,
    uri: Uri,
) -> Result<QueryResult<Services>, ResponseError> {
    let state = state.lock().await;

    Ok(state
        .delete(Some(HashMap::from([("id", id.into())])))
        .await
        .map_err(|x| Into::<Builder>::into(ResponseError::from(x)).instance(uri.to_string()))?)
}

pub async fn get(
    State(state): State<RepositoryType>,
    uri: Uri,
    Path(id): Path<Option<uuid::Uuid>>,
) -> Result<QueryResult<Services>, ResponseError> {
    let state = state.lock().await;

    Ok(state
        .get(id.map(|x| HashMap::from([("id", x.into())])))
        .await
        .map_err(|x| Into::<Builder>::into(ResponseError::from(x)).instance(uri.to_string()))?
        .into())
}
