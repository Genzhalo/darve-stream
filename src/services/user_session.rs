use tower_sessions::Session;

use crate::{interfaces::user_session::TUserSession, models::auth::AuthResponse};

const USER_KEY: &str = "user";

#[async_trait::async_trait]

impl TUserSession for Session {
    async fn get_user(&self) -> Result<AuthResponse, String> {
        let user = self
            .get::<AuthResponse>(USER_KEY)
            .await
            .map_err(|e| e.to_string())?;
        user.ok_or_else(|| "No user in session".to_string())
    }

    async fn is_authenticated(&self) -> Result<bool, String> {
        Ok(TUserSession::get_user(self).await.is_ok())
    }

    async fn set_user(&self, user: AuthResponse) -> Result<(), String> {
        self.insert(USER_KEY, user).await.map_err(|e| e.to_string())
    }

    async fn clear(&self) -> Result<(), String> {
        self.remove::<AuthResponse>(USER_KEY)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}
