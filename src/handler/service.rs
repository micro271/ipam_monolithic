use super::{RepositoryType, ResponseError};
use crate::{
    database::repository::{QueryResult, Repository},
    models::service::Service,
};
use axum::{extract::State, Json};

pub async fn create(
    State(state): State<RepositoryType>,
    Json(service): Json<Service>,
) -> Result<QueryResult<Service>, ResponseError> {
    let state = state.lock().await;

    Ok(state.insert::<Service>(vec![service]).await?)
}
