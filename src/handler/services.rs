use axum::{extract::State, response::IntoResponse, Json};
use crate::models::service::*;
use super::RepositoryType;



fn create(State(state): State<RepositoryType>, Json(service): Json<Service>) -> Result<impl IntoResponse, RepositoryType>{
    unimplemented!()
}