use askama::Template;
use serde::Serialize;

#[derive(Template, Serialize, Debug)]
#[template(path = "login_page.html")]
pub struct StreamLoginPage<'a> {
    pub twitch_client_id: &'a str,
    pub twitch_redirect_uri: &'a str,
    pub twitch_state: &'a str,
}
