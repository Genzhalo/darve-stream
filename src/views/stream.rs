use askama::Template;

use crate::models::auth::User;

#[derive(Template)]
#[template(path = "stream_page.html")]
pub struct StreamPage<'a> {
    pub user: &'a User,
}
