use askama::Template;
use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};
use tower_sessions::Session;

use crate::{interfaces::user_session::TUserSession, state::AppState, views::donate::DonatePage};

pub fn router() -> Router<AppState> {
    Router::new().route("/donate", get(donate))
}

async fn donate(session: Session, State(_state): State<AppState>) -> impl IntoResponse {
    let res = session.get_user().await.unwrap();
    let body = DonatePage { user: &res.user }.render().unwrap();
    Html(body).into_response()
}
