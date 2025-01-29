use axum::response::{Html, IntoResponse};
use axum_extra::routing::TypedPath;

use crate::error::AppError;

#[derive(TypedPath)]
#[typed_path("/dashboard")]
pub struct DashboardShowRoute;

pub async fn show(_: DashboardShowRoute) -> Result<impl IntoResponse, AppError> {
    Ok(Html("Welcome to the dashboard!"))
}
