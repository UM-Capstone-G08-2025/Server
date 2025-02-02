use axum::response::{Html, IntoResponse};
use rinja::Template;

use crate::error::AppError;

#[derive(Template)]
#[template(path = "error/404.html.jinja")]
pub struct Error404;

pub async fn error404() -> Result<impl IntoResponse, AppError> {
    Ok(Html(Error404.render()?))
}
