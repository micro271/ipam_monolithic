use tera::{Tera, Context};
use std::sync::LazyLock;
use axum::{extract::Request, response::{Html, IntoResponse}};
use tokio::sync::Mutex;
static TEMPLATES: LazyLock<Mutex<Tera>> = LazyLock::new(|| {
    Mutex::new(Tera::new("templates/**/*").expect("template dir doesn't found"))
});


pub async fn login() -> impl IntoResponse {
    let tera = TEMPLATES.lock().await;

    let context = Context::new();

    Html(tera.render("login.html", &context).unwrap()).into_response()
}


pub async fn fallback(req: Request) -> impl IntoResponse {
    let tera = TEMPLATES.lock().await;


    let mut context = Context::new();
    context.insert("path", &req.uri().to_string());

    Html(tera.render("fallback.html", &context).unwrap()).into_response()
}