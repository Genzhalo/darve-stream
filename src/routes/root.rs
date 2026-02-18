use axum::{
    Router,
    extract::State,
    response::{IntoResponse, Redirect},
    routing::get,
};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(root))
}

async fn root(State(_state): State<AppState>) -> impl IntoResponse {
    return Redirect::to("/dashboard").into_response();
}
