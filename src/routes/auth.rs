use crate::{
    interfaces::user_session::TUserSession, models::auth::AuthResponse, state::AppState,
    views::login::StreamLoginPage,
};
use askama::Template;
use axum::{
    Router,
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::get,
};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/auth/twitch/login", get(login))
        .route("/auth/twitch/callback", get(callback))
}

pub fn logout_router() -> Router<AppState> {
    Router::new().route("/auth/logout", get(logout))
}

async fn login(State(state): State<AppState>) -> impl IntoResponse {
    let body = StreamLoginPage {
        twitch_client_id: &state.config.twitch_client_id,
        twitch_redirect_uri: &state.config.twitch_redirect_uri,
        twitch_state: "TODO_GENERATE_RANDOM_STATE",
    }
    .render()
    .unwrap();
    Html(body.clone()).into_response()
}

#[derive(Debug, Deserialize, Serialize)]
struct GetTwitchQuery {
    code: String,
}

async fn callback(
    session: Session,
    State(app_state): State<AppState>,
    Query(params): Query<GetTwitchQuery>,
) -> impl IntoResponse {
    let client = reqwest::Client::new();
    let auth_url = format!("{}/auth/sign_with_twitch", app_state.config.api_base_url);
    let response = client
        .post(&auth_url)
        .json(&serde_json::json!({ "code": params.code }))
        .send()
        .await;

    match response {
        Ok(res) => {
            let data = res.json::<AuthResponse>().await.unwrap();
            println!("Auth response: {:?}", data);
            let _ = session.set_user(data).await.unwrap();
            Redirect::to("/").into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn logout(session: Session) -> impl IntoResponse {
    let _ = session.clear().await;
    Redirect::to("/auth/twitch/login").into_response()
}
