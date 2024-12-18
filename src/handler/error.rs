use crate::database::repository::error::RepositoryError;
use axum::http::StatusCode;
use libipam::response_error::ResponseError;

impl From<RepositoryError> for ResponseError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::Sqlx(e) => ResponseError::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .title(StatusCode::INTERNAL_SERVER_ERROR.to_string())
                .detail(e.to_string())
                .build(),
            RepositoryError::RowNotFound => ResponseError::builder()
                .status(StatusCode::NO_CONTENT)
                .title(StatusCode::NO_CONTENT.to_string())
                .detail("The provided data did not match any records in the table".to_string())
                .build(),
            RepositoryError::ColumnNotFound(e) => ResponseError::builder()
                .status(StatusCode::BAD_REQUEST)
                .title(StatusCode::BAD_REQUEST.to_string())
                .detail(e.unwrap_or_default() /* TODO */)
                .build(),
        }
    }
}
