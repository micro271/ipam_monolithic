pub mod utils;
pub mod convert;

use futures::stream::StreamExt;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqliteRow};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    str::FromStr,
};
use utils::*;
use crate::models::utils::*;

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
        sqlx::query(query).execute(&self.0).await?;
        Ok(())
    }

    async fn create_default_user(&self) -> Result<(), RepositoryError> {
        use crate::{services::encrypt, models::user::*};

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
            password: encrypt(
                std::env::var("IPAM_PASSWORD_ROOT")
                    .unwrap_or("admin".into())
                    .as_ref(),
            )
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
    fn insert<'a, T>(&'a self, data: Vec<T>) -> ResultRepository<'a, QueryResult>
    where
        T: Table + 'a + Send,
    {
        let resp = async {
            let mut tx = match self.0.begin().await {
                Ok(e) => e,
                Err(e) => return Err(RepositoryError::Sqlx(e.to_string())),
            };

            let mut count = 0;
            for data in data {
                let query = T::query_insert();
                let mut tmp = sqlx::query(&query);
                let data = T::get_fields(data);

                for i in data {
                    tmp = match i {
                        TypeTable::String(s) => tmp.bind(s),
                        TypeTable::OptionString(opt) => tmp.bind(opt),
                        TypeTable::OptionVlan(e) => tmp.bind(e),
                        TypeTable::Status(status) => tmp.bind(status),
                        TypeTable::Int32(i) => tmp.bind(i),
                        TypeTable::Uuid(e) => tmp.bind(e),
                        TypeTable::Role(r) => tmp.bind(r),
                        TypeTable::Float64(f) => tmp.bind(f),
                        TypeTable::OptionUuid(e) => tmp.bind(e),
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
                Ok(_) => Ok(QueryResult::Insert(count)),
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
        T: Table + From<SqliteRow> + 'a + Send,
    {
        Box::pin(async {
            let mut query = format!("SELECT * FROM {}", T::name());
            let mut vec_resp = Vec::new();

            match column_data {
                Some(col) if !col.is_empty() => {
                    let cols = T::columns();
                    query.push_str(" WHERE");

                    let mut data_pos = HashMap::new();

                    let mut pos = 1;
                    for i in col.keys() {
                        if !cols.contains(i) {
                            return Err(RepositoryError::ColumnNotFound(Some(i.to_string())));
                        }

                        query.push_str(&format!(" {} = ${}", i, pos));

                        data_pos.insert(pos, col.get(i).unwrap());
                        pos += 1;
                    }

                    let mut resp = sqlx::query(&query);

                    for i in 1..pos {
                        resp = match data_pos.get(&i).unwrap() {
                            TypeTable::OptionUuid(e) => resp.bind(e),
                            TypeTable::OptionVlan(e) => resp.bind(e),
                            TypeTable::Uuid(e) => resp.bind(e),
                            TypeTable::String(s) => resp.bind(s),
                            TypeTable::OptionString(opt) => resp.bind(opt),
                            TypeTable::Status(status) => resp.bind(status),
                            TypeTable::Int32(num) => resp.bind(num),
                            TypeTable::Role(role) => resp.bind(role),
                            TypeTable::Float64(f) => resp.bind(f),
                        };
                    }

                    let mut resp = resp.fetch(&self.0);
                    while let Some(Ok(device)) = resp.next().await {
                        vec_resp.push(T::from(device));
                    }

                    if !vec_resp.is_empty() {
                        Ok(vec_resp)
                    } else {
                        Err(RepositoryError::RowNotFound)
                    }
                }
                None => Ok({
                    vec_resp.push(T::from(sqlx::query(&query).fetch_one(&self.0).await?));
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
    ) -> ResultRepository<'a, QueryResult>
    where
        T: Table + 'a + Send,
        U: Updatable<'a> + 'a + Send,
    {
        let tmp = async move {
            if let Some(pair) = updater.get_pair() {
                let cols = T::columns();

                let mut query = format!("UPDATE {} SET", T::name());

                let mut pos_values = HashMap::new();

                let mut pos = 1;
                for i in pair.keys() {
                    if !cols.contains(i) {
                        return Err(RepositoryError::ColumnNotFound(Some(i.to_string())));
                    }

                    query.push_str(&format!(" {} = ${}", i, pos));
                    pos_values.insert(pos, pair.get(i).unwrap());
                    pos += 1
                }

                let condition = match condition {
                    Some(e) => {
                        query.push_str(" WHERE");
                        e
                    }
                    None => HashMap::new(),
                };

                for i in condition.keys() {
                    pos_values.insert(pos, condition.get(i).unwrap());
                    query.push_str(&format!(" {} = ${}", i, pos));
                    pos += 1;
                }

                let mut sql = sqlx::query(&query);
                for i in 1..pos {
                    sql = match pos_values.get(&i).unwrap() {
                        TypeTable::String(s) => sql.bind(s),
                        TypeTable::OptionString(value) => sql.bind(value),
                        TypeTable::Status(value) => sql.bind(value),
                        TypeTable::Int32(value) => sql.bind(value),
                        TypeTable::Uuid(e) => sql.bind(e),
                        TypeTable::Role(value) => sql.bind(value),
                        TypeTable::Float64(value) => sql.bind(value),
                        TypeTable::OptionUuid(e) => sql.bind(e),
                        TypeTable::OptionVlan(e) => sql.bind(e),
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
    ) -> ResultRepository<'a, QueryResult>
    where
        T: Table + 'a + Send,
    {
        let resp = async move {
            let mut query = format!("DELETE FROM {}", T::name());

            match condition {
                Some(condition) if !condition.is_empty() => {
                    let columns = T::columns();

                    query.push_str(" WHERE");

                    let mut pos_column = HashMap::new();
                    let mut pos = 1;

                    for t in condition.keys() {
                        if !columns.contains(t) {
                            return Err(RepositoryError::ColumnNotFound(Some(t.to_string())));
                        }

                        query.push_str(&format!(" {} = ${}", t, pos));
                        pos_column.insert(pos, condition.get(t).unwrap());
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
                            TypeTable::Int32(i) => ex.bind(i),
                            TypeTable::Role(role) => ex.bind(role),
                            TypeTable::Float64(f) => ex.bind(f),
                            TypeTable::OptionVlan(e) => ex.bind(e),
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
