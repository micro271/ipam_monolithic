use super::Table;
use crate::models::utils::{TypeTable, Updatable};
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
        T: Table + From<SqliteRow> + 'a + Send + Debug;
    fn insert<'a, T>(&'a self, data: Vec<T>) -> ResultRepository<'a, QueryResult>
    where
        T: Table + 'a + Send + Debug;
    fn update<'a, T, U>(
        &'a self,
        updater: U,
        condition: Option<HashMap<&'a str, TypeTable>>,
    ) -> ResultRepository<'a, QueryResult>
    where
        T: Table + 'a + Send + Debug,
        U: Updatable<'a> + Send + 'a + Debug;
    fn delete<'a, T>(
        &'a self,
        condition: Option<HashMap<&'a str, TypeTable>>,
    ) -> ResultRepository<'a, QueryResult>
    where
        T: Table + 'a + Send + Debug;
}

#[derive(Debug)]
pub enum RepositoryError {
    Sqlx(String),
    RowNotFound,
    //    Unauthorized(String),
    ColumnNotFound(Option<String>),
}

pub enum QueryResult {
    Insert(u64),
    Update(u64),
    Delete(u64),
}

impl QueryResult {
    pub fn unwrap(self) -> u64 {
        match self {
            QueryResult::Insert(e) => e,
            QueryResult::Update(e) => e,
            QueryResult::Delete(e) => e,
        }
    }
}
