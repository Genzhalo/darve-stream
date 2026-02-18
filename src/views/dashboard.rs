use askama::Template;

use crate::models::auth::User;

#[derive(Template)]
#[template(path = "dashboard_page.html")]
pub struct DashboardPage<'a> {
    pub user: &'a User,
}
