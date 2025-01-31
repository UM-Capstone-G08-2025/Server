use axum::{
    extract::Request,
    middleware::Next,
    response::{Html, IntoResponse, Redirect, Response},
};
use axum_extra::routing::TypedPath;
use http::StatusCode;
use rinja::Template;
use tower_cookies::{Cookie, Cookies};
use tracing::info;

use crate::{
    error::AppError,
    features::{error::Error404, session::SessionCreateRoute},
};

const COOKIE_NAME: &str = "session_id";

pub async fn handle(cookies: Cookies, request: Request, next: Next) -> Result<Response, AppError> {
    let response = if let Some(session_id) = cookies.get(COOKIE_NAME) {
        info!("Has cookie");
        next.run(request).await.into_response()
    } else {
        Html(Error404.render()?).into_response()
    };

    Ok(response)
}
