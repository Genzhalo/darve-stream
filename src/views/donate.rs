use askama::Template;

use crate::models::auth::User;

#[derive(Template)]
#[template(path = "donate_page.html")]
pub struct DonatePage<'a> {
    pub user: &'a User,
}
