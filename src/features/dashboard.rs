use axum::response::{Html, IntoResponse};
use axum_extra::routing::TypedPath;
use rinja::Template;

use crate::error::AppError;

#[derive(TypedPath)]
#[typed_path("/dashboard")]
pub struct DashboardShowRoute;

pub async fn show(_: DashboardShowRoute) -> Result<impl IntoResponse, AppError> {
    Ok(Html(DashboardShowTemplate.render()?))
}

#[derive(Template)]
#[template(path = "dashboard/show.html.jinja")]
struct DashboardShowTemplate;
