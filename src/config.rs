use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub twitch_client_id: String,
    pub twitch_redirect_uri: String,
    pub api_base_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            twitch_client_id: env::var("TWITCH_CLIENT_ID").expect("TWITCH_CLIENT_ID must be set"),
            twitch_redirect_uri: env::var("TWITCH_REDIRECT_URI")
                .expect("TWITCH_REDIRECT_URI must be set"),
            api_base_url: env::var("API_BASE_URL").expect("API_BASE_URL must be set"),
        }
    }
}
