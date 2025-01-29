use axum::{extract::Request, middleware::Next, response::Response};
use tower_cookies::{Cookie, Cookies};
use tracing::info;

use crate::error::AppError;

const COOKIE_NAME: &str = "session_id";

pub async fn handle(cookies: Cookies, request: Request, next: Next) -> Result<Response, AppError> {
    if let Some(session_id) = cookies.get(COOKIE_NAME) {
        info!("Has cookie");
    } else {
        cookies.add(Cookie::new(COOKIE_NAME, "test"));
        info!("Inserted new cookie");
    }

    let response = next.run(request).await;

    Ok(response)
}
