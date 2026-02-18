use crate::models::auth::AuthResponse;

#[async_trait::async_trait]
pub trait TUserSession {
    async fn get_user(&self) -> Result<AuthResponse, String>;
    async fn is_authenticated(&self) -> Result<bool, String>;
    async fn set_user(&self, user: AuthResponse) -> Result<(), String>;
    async fn clear(&self) -> Result<(), String>;
}
