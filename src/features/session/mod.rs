use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
    Form,
};
use axum_extra::routing::TypedPath;
use rinja::Template;
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};
use tracing::info;

use crate::{error::AppError, features::dashboard::DashboardShowRoute, state::AppState};

#[derive(TypedPath)]
#[typed_path("/session")]
pub struct SessionStoreRoute;

#[derive(Deserialize)]
pub struct SessionCreateRequest {
    username: String,
    password: String,
}

pub async fn store(
    _: SessionStoreRoute,
    cookies: Cookies,
    State(state): State<AppState>,
    Form(login_request): Form<SessionCreateRequest>,
) -> impl IntoResponse {
    if login_request.username == state.config.auth.username
        && login_request.password == state.config.auth.password
    {
        cookies.add(Cookie::new("session_id", "test"));
        info!("successful login");

        return Redirect::to(&DashboardShowRoute.to_string());
    } else {
        return Redirect::to(&SessionCreateRoute.to_string());
    }
}

#[derive(TypedPath)]
#[typed_path("/login")]
pub struct SessionCreateRoute;

pub async fn create(_: SessionCreateRoute) -> Result<impl IntoResponse, AppError> {
    Ok(Html(SessionCreateTemplate.render()?))
}

#[derive(Template)]
#[template(path = "session/create.jinja")]
struct SessionCreateTemplate;
