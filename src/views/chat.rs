use askama::Template;

use crate::models::auth::User;

#[derive(Template)]
#[template(path = "chat_page.html")]
pub struct ChatPage<'a> {
    pub user: &'a User,
}
