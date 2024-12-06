pub mod convert;
pub mod repository;

use crate::models::utils::*;
use futures::stream::StreamExt;
use repository::{error::RepositoryError, QueryResult, Repository, ResultRepository};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqliteRow};
use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[derive(Debug)]
pub struct SqliteRepository(SqlitePool);

impl SqliteRepository {
    pub async fn new(url: &str) -> Result<Self, RepositoryError> {
        let mut create_tables = false;

        let path_db = std::path::Path::new(url);

        if !path_db.exists() {
            std::fs::File::create(path_db).expect("Database can't create");
            create_tables = true;
        }

        let db = Self({
            let tmp = SqliteConnectOptions::from_str(url)?
                .journal_mode(SqliteJournalMode::Wal)
                .read_only(false);
            SqlitePool::connect_with(tmp).await?
        });

        if create_tables {
            db.init_db().await?;
            db.create_default_user().await?;
        }

        Ok(db)
    }

    async fn init_db(&self) -> Result<(), RepositoryError> {
        let query = include_str!("../../initdb.sql");
        sqlx::query(query)
            .execute(&self.0)
            .await
            .expect("Database error");
        Ok(())
    }

    async fn create_default_user(&self) -> Result<(), RepositoryError> {
        use crate::models::user::*;
        use libipam::authentication::encrypt;

        if self
            .get::<User>(Some(HashMap::from([("role", Role::Admin.into())])))
            .await
            .is_ok()
        {
            return Ok(());
        }

        let user = User {
            id: uuid::Uuid::new_v4(),
            username: std::env::var("IPAM_USER_ROOT").unwrap_or("admin".into()),
            password: encrypt(std::env::var("IPAM_PASSWORD_ROOT").unwrap_or("admin".into()))
                .expect("Encrypt default user error"),
            role: Role::Admin,
        };

        match self.insert::<User>(vec![user]).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

impl Repository for SqliteRepository {
    fn insert<'a, T>(&'a self, data: Vec<T>) -> ResultRepository<'a, QueryResult<T>>
    where
        T: Table + 'a + Send + Debug + Clone,
    {
        let resp = async {
            let mut tx = match self.begin().await {
                Ok(e) => e,
                Err(e) => return Err(RepositoryError::Sqlx(e.to_string())),
            };
            let mut resp_data = Vec::new();

            let mut count = 0;
            for data in data {
                resp_data.push(data.clone());
                let query = T::query_insert();
                let mut tmp = sqlx::query(&query);
                let data = T::get_fields(data);
                for i in data {
                    tmp = match i {
                        TypeTable::String(s) => tmp.bind(s),
                        TypeTable::OptionString(opt) => tmp.bind(opt),
                        TypeTable::OptionI32(e) => tmp.bind(e),
                        TypeTable::Status(status) => tmp.bind(status),
                        TypeTable::U32(i) => tmp.bind(i),
                        TypeTable::Uuid(e) => tmp.bind(e),
                        TypeTable::Role(r) => tmp.bind(r),
                        TypeTable::OptionUuid(e) => tmp.bind(e),
                        TypeTable::BytesOption(e) => tmp.bind(e),
                        TypeTable::Null => tmp,
                        TypeTable::U16(e) =>  tmp.bind(e),
                    };
                }

                match tmp.execute(&mut *tx).await {
                    Ok(_) => {
                        count += 1;
                    }
                    Err(e) => {
                        tx.rollback().await?;
                        return Err(RepositoryError::Sqlx(e.to_string()));
                    }
                }
            }

            match tx.commit().await {
                Ok(_) => Ok(QueryResult::Insert {
                    row_affect: count,
                    data: resp_data,
                }),
                Err(e) => Err(RepositoryError::Sqlx(e.to_string())),
            }
        };
        Box::pin(resp)
    }

    fn get<'a, T>(
        &'a self,
        column_data: Option<HashMap<&'a str, TypeTable>>,
    ) -> ResultRepository<'a, Vec<T>>
    where
        T: Table + From<SqliteRow> + 'a + Send + Debug,
    {
        Box::pin(async {
            let mut query = format!("SELECT * FROM {}", T::name());
            let mut vec_resp = Vec::new();
            tracing::debug!("Get element % Condition select {:?} %", column_data);
            match column_data {
                Some(col) if !col.is_empty() => {
                    let cols = T::columns();
                    query.push_str(" WHERE");

                    let mut data_pos = HashMap::new();

                    let mut pos = 1;
                    let len = col.len();
                    for i in col.keys() {
                        if !cols.contains(i) {
                            return Err(RepositoryError::ColumnNotFound(Some(i.to_string())));
                        }
                        if col.get(i).unwrap() == &TypeTable::Null {
                            query.push_str(&format!(" {} IS NULL", i));
                        } else {
                            query.push_str(&format!(" {} = ${}", i, pos));
                            if pos < len {
                                query.push_str(" AND");
                            }
                            data_pos.insert(pos, col.get(i).unwrap());
                            pos += 1;
                        }
                    }
                    tracing::debug!("{}", query);
                    tracing::debug!("{:?}", data_pos);
                    let mut resp = sqlx::query(&query);

                    for i in 1..pos {
                        resp = match data_pos.get(&i).unwrap() {
                            TypeTable::OptionUuid(e) => resp.bind(e),
                            TypeTable::OptionI32(e) => resp.bind(e),
                            TypeTable::Uuid(e) => resp.bind(e),
                            TypeTable::String(s) => resp.bind(s),
                            TypeTable::OptionString(opt) => resp.bind(opt),
                            TypeTable::Status(status) => resp.bind(status),
                            TypeTable::U32(num) => resp.bind(num),
                            TypeTable::Role(role) => resp.bind(role),
                            TypeTable::BytesOption(e) => resp.bind(e),
                            TypeTable::Null => resp,
                            TypeTable::U16(e) => resp.bind(e),
                        };
                    }

                    let mut resp = resp.fetch(&self.0);
                    while let Some(Ok(device)) = resp.next().await {
                        vec_resp.push(T::from(device));
                    }
                    tracing::debug!("{:?}", vec_resp);
                    if !vec_resp.is_empty() {
                        Ok(vec_resp)
                    } else {
                        Err(RepositoryError::RowNotFound)
                    }
                }
                None => Ok({
                    let mut fetch = sqlx::query(&query).fetch(&self.0);
                    while let Some(Ok(tmp)) = fetch.next().await {
                        vec_resp.push(tmp.into());
                    }

                    vec_resp
                }),
                _ => Err(RepositoryError::ColumnNotFound(None)),
            }
        })
    }

    fn update<'a, T, U>(
        &'a self,
        updater: U,
        condition: Option<HashMap<&'a str, TypeTable>>,
    ) -> ResultRepository<'a, QueryResult<T>>
    where
        T: Table + 'a + Send + Debug,
        U: Updatable<'a> + 'a + Send + Debug,
    {
        let tmp = async move {
            tracing::debug!(
                "Update element % new_data: {:?} - condition {:?} %",
                updater,
                condition
            );
            if let Some(pair) = updater.get_pair() {
                let cols = T::columns();

                let mut query = format!("UPDATE {} SET", T::name());

                let mut pos_values = HashMap::new();

                let mut pos = 1;
                let len = pair.len();
                for i in pair.keys() {
                    if !cols.contains(i) {
                        return Err(RepositoryError::ColumnNotFound(Some(i.to_string())));
                    }

                    query.push_str(&format!(" {} = ${}", i, pos));
                    pos_values.insert(pos, pair.get(i).unwrap());
                    if len > pos {
                        query.push(',');
                    }
                    pos += 1
                }

                let condition = match condition {
                    Some(e) => {
                        query.push_str(" WHERE");
                        e
                    }
                    None => HashMap::new(),
                };

                let len = condition.len() + pos - 1;
                for i in condition.keys() {
                    pos_values.insert(pos, condition.get(i).unwrap());
                    query.push_str(&format!(" {} = ${}", i, pos));
                    if pos < len {
                        query.push_str(" AND");
                    }
                    pos += 1;
                }

                let mut sql = sqlx::query(&query);
                for i in 1..pos {
                    sql = match pos_values.get(&i).unwrap() {
                        TypeTable::String(s) => sql.bind(s),
                        TypeTable::OptionString(value) => sql.bind(value),
                        TypeTable::Status(value) => sql.bind(value),
                        TypeTable::Uuid(e) => sql.bind(e),
                        TypeTable::Role(value) => sql.bind(value),
                        TypeTable::OptionUuid(e) => sql.bind(e),
                        TypeTable::OptionI32(e) => sql.bind(e),
                        TypeTable::BytesOption(e) => sql.bind(e),
                        TypeTable::U32(e) => sql.bind(e),
                        TypeTable::Null => sql,
                        TypeTable::U16(e) => sql.bind(e),
                    };
                }

                match sql.execute(&self.0).await {
                    Ok(e) => Ok(QueryResult::Update(e.rows_affected())),
                    Err(e) => Err(RepositoryError::Sqlx(e.to_string())),
                }
            } else {
                Err(RepositoryError::ColumnNotFound(None))
            }
        };
        Box::pin(tmp)
    }

    fn delete<'a, T>(
        &'a self,
        condition: Option<HashMap<&'a str, TypeTable>>,
    ) -> ResultRepository<'a, QueryResult<T>>
    where
        T: Table + 'a + Send + Debug,
    {
        let resp = async move {
            let mut query = format!("DELETE FROM {}", T::name());

            match condition {
                Some(condition) if !condition.is_empty() => {
                    let columns = T::columns();

                    query.push_str(" WHERE");

                    let mut pos_column = HashMap::new();
                    let mut pos = 1;

                    let len = condition.len();
                    for t in condition.keys() {
                        if !columns.contains(t) {
                            return Err(RepositoryError::ColumnNotFound(Some(t.to_string())));
                        }

                        query.push_str(&format!(" {} = ${}", t, pos));
                        pos_column.insert(pos, condition.get(t).unwrap());
                        if pos < len {
                            query.push_str(" AND");
                        }
                        pos += 1;
                    }

                    let mut ex = sqlx::query(&query);

                    for i in 1..pos {
                        ex = match pos_column.get(&i).unwrap() {
                            TypeTable::OptionUuid(e) => ex.bind(e),
                            TypeTable::String(s) => ex.bind(s),
                            TypeTable::OptionString(s) => ex.bind(s),
                            TypeTable::Uuid(e) => ex.bind(e),
                            TypeTable::Status(status) => ex.bind(status),
                            TypeTable::BytesOption(e) => ex.bind(e),
                            TypeTable::Role(role) => ex.bind(role),
                            TypeTable::OptionI32(e) => ex.bind(e),
                            TypeTable::U32(i) => ex.bind(i),
                            TypeTable::Null => ex,
                            TypeTable::U16(e) => ex.bind(e),
                        };
                    }

                    match ex.execute(&self.0).await {
                        Ok(e) => Ok(QueryResult::Delete(e.rows_affected())),
                        Err(e) => Err(RepositoryError::Sqlx(e.to_string())),
                    }
                }

                None => match sqlx::query(&query).execute(&self.0).await {
                    Ok(e) => Ok(QueryResult::Delete(e.rows_affected())),
                    Err(e) => Err(RepositoryError::Sqlx(e.to_string())),
                },
                _ => Err(RepositoryError::ColumnNotFound(None)),
            }
        };

        Box::pin(resp)
    }
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::Sqlx(txt) => write!(f, "Sqlx error: {}", txt),
            Self::RowNotFound => write!(f, "Row doesn't exist"),
            Self::ColumnNotFound(e) => match e {
                Some(e) => {
                    write!(f, "The column {} didn't find", e)
                }
                None => {
                    write!(f, "Undefined collumn")
                }
            },
        }
    }
}

impl std::error::Error for RepositoryError {}

impl From<sqlx::Error> for RepositoryError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::RowNotFound,
            sqlx::Error::ColumnNotFound(e) => Self::ColumnNotFound(Some(e)),
            e => Self::Sqlx(e.to_string()),
        }
    }
}

impl Deref for SqliteRepository {
    type Target = SqlitePool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SqliteRepository {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
