use crate::services::Claims;

use super::*;
use axum::{extract::Request, middleware::Next, response::Response};
use libipam::authentication::{self, create_token, encrypt, verify_passwd};

pub async fn create(
    State(state): State<RepositoryType>,
    Extension(role): Extension<Role>,
    Json(mut user): Json<user::User>,
) -> Result<impl IntoResponse, ResponseError> {
    if role != Role::Admin {
        return Err(ResponseError::Unauthorized);
    }

    let state = state.lock().await;

    user.password = match encrypt(user.password) {
        Ok(e) => e,
        Err(_) => return Err(ResponseError::ServerError),
    };

    Ok(state.insert(vec![user]).await?)
}

pub async fn login(
    State(state): State<RepositoryType>,
    Json(user): Json<models_data_entry::User>,
) -> Result<impl IntoResponse, ResponseError> {
    let state = state.lock().await;

    let resp = state
        .get::<'_, user::User>(Some(HashMap::from([("username", user.username.into())])))
        .await?
        .remove(0);

    if verify_passwd(user.password, &resp.password) {
        match create_token(Claims::from(resp)) {
            Ok(e) => Ok(Json(json!({"token":e}))),
            Err(_) => Err(ResponseError::ServerError),
        }
    } else {
        Err(ResponseError::Unauthorized)
    }
}

pub async fn verify_token(mut req: Request, next: Next) -> Result<Response, ResponseError> {
    match req.headers().get(axum::http::header::AUTHORIZATION) {
        Some(e) => match e.to_str() {
            Ok(e) => match e.split(' ').collect::<Vec<_>>().get(1) {
                Some(e) => match authentication::verify_token::<Claims, _>(*e) {
                    Ok(e) => {
                        req.extensions_mut().insert(e.role);
                        Ok(next.run(req).await)
                    }
                    _ => Err(ResponseError::Unauthorized),
                },
                None => Err(ResponseError::StatusCode(StatusCode::BAD_REQUEST)),
            },
            Err(_) => Err(ResponseError::StatusCode(StatusCode::BAD_REQUEST)),
        },
        None => Err(ResponseError::Unauthorized),
    }
}
