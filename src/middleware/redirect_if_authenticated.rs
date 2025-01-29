use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::routing::TypedPath;
use tower_cookies::Cookies;
use tracing::info;

use crate::{error::AppError, features::dashboard::DashboardShowRoute};

const COOKIE_NAME: &str = "session_id";

pub async fn handle(cookies: Cookies, request: Request, next: Next) -> Result<Response, AppError> {
    let response = if let Some(session_id) = cookies.get(COOKIE_NAME) {
        info!("Has cookie. Redirecting");

        Redirect::to(&DashboardShowRoute.to_uri().to_string()).into_response()
    } else {
        next.run(request).await.into_response()
    };

    Ok(response)
}
