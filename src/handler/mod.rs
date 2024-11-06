pub mod device;
pub mod error;
mod models_data_entry;
pub mod network;
pub mod auth;
mod query_params;

use crate::database::{repository::Repository, SqliteRepository};
use crate::models::{*, user::Role};
use axum::{
    extract::{Extension, Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use error::ResponseError;
use serde_json::json;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;


type RepositoryType = Arc<Mutex<SqliteRepository>>;
