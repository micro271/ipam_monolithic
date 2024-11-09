use crate::services::Claims;

use super::*;
use axum::{extract::Request, middleware::Next, response::{Redirect, Response}};
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
            Ok(e) => {

                let mut req = Redirect::to("/").into_response();
                let cook = cookie::Cookie::build((libipam::cookie::Cookie::TOKEN.to_string(), e))
                    .http_only(true)
                    .path("/")
                    .max_age(time::Duration::minutes(30));
                req.headers_mut().insert(axum::http::header::SET_COOKIE, cook.to_string().parse().unwrap());
                Ok(req)
            },
            Err(_) => Err(ResponseError::ServerError),
        }
    } else {
        Err(ResponseError::Unauthorized)
    }
}

pub async fn verify_token(libipam::Token(token): libipam::Token, mut req: Request, next: Next) -> Result<Response, Redirect> {
    match authentication::verify_token::<Claims,_>(token) {
        Ok(e) => {
            req.extensions_mut().insert(e.role);
            Ok(next.run(req).await)
        },
        Err(_) => Err(Redirect::to("/login")),
    }
}
