use super::*;
use crate::services::Claims;
use axum::{
    extract::Request,
    middleware::Next,
    response::{Redirect, Response},
};
use libipam::authentication::{self, create_token, encrypt, verify_passwd};

#[axum::debug_handler]
pub async fn create(
    State(state): State<RepositoryType>,
    Extension(claim): Extension<Claims>,
    uri: Uri,
    Json(mut user): Json<user::User>,
) -> Result<impl IntoResponse, ResponseError> {
    if claim.role != Role::Admin {
        return Err(ResponseError::builder()
            .status(StatusCode::UNAUTHORIZED)
            .instance(uri.to_string())
            .detail(format!(
                "The user {} doesn't belong to the {:?} role",
                claim.sub,
                Role::Admin
            ))
            .title("User not authorized".to_string())
            .build());
    }

    let state = state.lock().await;

    user.password = match encrypt(user.password) {
        Ok(e) => e,
        Err(e) => {
            return Err(ResponseError::builder()
                .status(StatusCode::UNAUTHORIZED)
                .instance(uri.to_string())
                .detail(e.to_string())
                .build())
        }
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

                req.headers_mut().insert(
                    axum::http::header::SET_COOKIE,
                    cook.to_string().parse().unwrap(),
                );
                Ok(req)
            }
            Err(e) => Err(ResponseError::builder()
                .title("We've had an error to create the token".to_string())
                .detail(e.to_string())
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .build()),
        }
    } else {
        Err(ResponseError::builder()
            .status(StatusCode::UNAUTHORIZED)
            .build())
    }
}

pub async fn verify_token(
    libipam::Token(token): libipam::Token,
    mut req: Request,
    next: Next,
) -> Result<Response, Redirect> {
    match token.map(authentication::verify_token::<Claims, _>) {
        Ok(Ok(e)) => {
            tracing::Span::current().record("id", tracing::field::display(e.sub));
            tracing::Span::current().record("role", tracing::field::debug(&e.role));
            tracing::Span::current().record("username", tracing::field::display(&e.username));
            req.extensions_mut().insert(e);
            Ok(next.run(req).await)
        }
        _ => Err(Redirect::to("/login")),
    }
}
