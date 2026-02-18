use axum::{
    Router,
    body::Body,
    http::Request,
    middleware::{self, Next},
    response::{IntoResponse, Redirect, Response},
};
use tower_sessions::Session;

use crate::{interfaces::user_session::TUserSession, state::AppState};

mod auth;
mod chat;
mod dashboard;
mod donate;
mod root;
mod stream;

pub fn router() -> Router<AppState> {
    let auth_public_router = auth::public_router().layer(middleware::from_fn(no_guard));
    let protected_router = Router::new()
        .merge(auth::logout_router())
        .merge(chat::router())
        .merge(donate::router())
        .merge(stream::router())
        .merge(dashboard::router())
        .merge(root::router())
        .layer(middleware::from_fn(auth_guard));

    Router::new()
        .merge(auth_public_router)
        .merge(protected_router)
}

async fn auth_guard(session: Session, req: Request<Body>, next: Next) -> Response {
    if session.is_authenticated().await.unwrap_or(false) {
        return next.run(req).await;
    }
    Redirect::to("/auth/twitch/login").into_response()
}

async fn no_guard(session: Session, req: Request<Body>, next: Next) -> Response {
    if session.is_authenticated().await.unwrap_or(false) {
        return Redirect::to("/dashboard").into_response();
    }
    next.run(req).await
}
