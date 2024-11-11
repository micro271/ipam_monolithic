use std::net::SocketAddr;
use axum::{body::Body, extract::Request, response::Response};
use tracing::{info, span::Span};
use axum::extract::ConnectInfo;


pub fn make_span(req :&Request<Body>) -> Span {
    let tmp = req.extensions().get::<ConnectInfo<SocketAddr>>().map(|conn| {
        tracing::field::display(conn.ip().to_string())
    }).unwrap_or(tracing::field::display(String::from("Unknown")));

    tracing::info_span!("http_log", uri = %req.uri(), method = %req.method(), peer = %tmp, latency = tracing::field::Empty,
    status = tracing::field::Empty) 
}

pub fn on_request(_req: &Request<Body>, _span: &Span) {
    tracing::debug!("Request")
}

pub fn on_response(req: &Response<Body>, dur: std::time::Duration, span: &Span) {
    span.record("latency", tracing::field::display(format!("{}ms", dur.as_millis())));
    span.record("status", tracing::field::display(req.status()));
    info!("Response")
}