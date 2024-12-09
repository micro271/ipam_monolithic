pub mod auth;
pub mod device;
pub mod error;
pub mod http;
mod models_data_entry;
pub mod network;
mod query_params;
pub mod service;
pub mod services;

use crate::database::{
    repository::{QueryResult, Repository},
    SqliteRepository,
};
use crate::models::{user::Role, *};
use crate::services::Claims;
use axum::{
    extract::{Extension, Json, Path, Query, State},
    http::{StatusCode, Uri},
    response::IntoResponse,
};
use libipam::response_error::{self, ResponseError};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

type RepositoryType = Arc<Mutex<SqliteRepository>>;
