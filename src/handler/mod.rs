pub mod auth;
pub mod device;
pub mod error;
pub mod http;
mod models_data_entry;
pub mod network;
mod query_params;

use crate::database::{repository::Repository, SqliteRepository};
use crate::models::{user::Role, *};
use crate::services::Claims;
use axum::{
    extract::{Extension, Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

type RepositoryType = Arc<Mutex<SqliteRepository>>;
