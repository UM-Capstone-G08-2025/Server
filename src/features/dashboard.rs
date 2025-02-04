use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use axum_extra::routing::TypedPath;
use rinja::Template;

use crate::{error::AppError, state::AppState};

#[derive(TypedPath)]
#[typed_path("/dashboard")]
pub struct DashboardShowRoute;

pub async fn show(
    _: DashboardShowRoute,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Html(
        DashboardShowTemplate {
            api_key: &state.config.auth.api_key,
        }
        .render()?,
    ))
}

#[derive(Template)]
#[template(path = "dashboard/show.html.jinja")]
struct DashboardShowTemplate<'a> {
    pub api_key: &'a str,
}
