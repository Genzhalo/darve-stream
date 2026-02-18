use askama::Template;
use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};
use tower_sessions::Session;

use crate::{
    interfaces::user_session::TUserSession, state::AppState, views::dashboard::DashboardPage,
};

pub fn router() -> Router<AppState> {
    Router::new().route("/dashboard", get(dashboard))
}

async fn dashboard(session: Session, State(_state): State<AppState>) -> impl IntoResponse {
    let res = session.get_user().await.unwrap();
    let body = DashboardPage { user: &res.user }.render().unwrap();
    Html(body).into_response()
}
