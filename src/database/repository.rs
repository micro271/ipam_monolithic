use super::Table;
use crate::models::utils::{TypeTable, Updatable};
use axum::{
    http::{Response, StatusCode},
    response::IntoResponse,
};
use error::RepositoryError;
use serde_json::json;
use sqlx::sqlite::SqliteRow;
use std::{boxed::Box, collections::HashMap, fmt::Debug, future::Future, pin::Pin};

pub type ResultRepository<'a, T> =
    Pin<Box<dyn Future<Output = Result<T, RepositoryError>> + 'a + Send>>;

pub trait Repository {
    fn get<'a, T>(
        &'a self,
        primary_key: Option<HashMap<&'a str, TypeTable>>,
    ) -> ResultRepository<'a, Vec<T>>
    where
        T: Table + From<SqliteRow> + 'a + Send + Debug + Clone;
    fn insert<'a, T>(&'a self, data: Vec<T>) -> ResultRepository<'a, QueryResult<T>>
    where
        T: Table + 'a + Send + Debug + Clone;
    fn update<'a, T, U>(
        &'a self,
        updater: U,
        condition: Option<HashMap<&'a str, TypeTable>>,
    ) -> ResultRepository<'a, QueryResult<T>>
    where
        T: Table + 'a + Send + Debug + Clone,
        U: Updatable<'a> + Send + 'a + Debug;
    fn delete<'a, T>(
        &'a self,
        condition: Option<HashMap<&'a str, TypeTable>>,
    ) -> ResultRepository<'a, QueryResult<T>>
    where
        T: Table + 'a + Send + Debug + Clone;
}

pub enum QueryResult<T> {
    Insert { row_affect: u64, data: Vec<T> },
    Update(u64),
    Delete(u64),
}

impl<S> IntoResponse for QueryResult<S>
where
    S: serde::Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let (body, status) = match self {
            Self::Insert { row_affect, data } => (
                json!({
                    "status": 201,
                    "row_affect": row_affect,
                    "data": data
                }),
                StatusCode::CREATED,
            ),
            Self::Update(e) | Self::Delete(e) => (
                json!({
                    "status": 200,
                    "row_affect": e
                }),
                StatusCode::OK,
            ),
        };

        Response::builder()
            .status(status)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body::<String>(body.to_string())
            .unwrap_or_default()
            .into_response()
    }
}

pub mod error {
    #[derive(Debug)]
    pub enum RepositoryError {
        Sqlx(String),
        RowNotFound,
        //    Unauthorized(String),
        ColumnNotFound(Option<String>),
    }
}
