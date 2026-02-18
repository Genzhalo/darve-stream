use askama::Template;
use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};
use tower_sessions::Session;

use crate::{interfaces::user_session::TUserSession, state::AppState, views::chat::ChatPage};

pub fn router() -> Router<AppState> {
    Router::new().route("/chat", get(chat))
}

async fn chat(session: Session, State(_state): State<AppState>) -> impl IntoResponse {
    let res = session.get_user().await.unwrap();
    let body = ChatPage { user: &res.user }.render().unwrap();
    Html(body).into_response()
}
