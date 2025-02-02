use crate::{
    features::{dashboard, error, session},
    middleware,
    // features::{authenticated, layout::wrap_in_layout},
    state::AppState,
};
use axum::{
    middleware::{from_fn, from_fn_with_state},
    routing::get,
    Router,
};
use axum_extra::routing::RouterExt;
use tower_cookies::CookieManagerLayer;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(public_routes())
        .merge(authenticated_routes(state))
        .fallback(error::error404)
        .layer(CookieManagerLayer::new())
        .route("/ping", get(|| async { "pong" }))
}

fn public_routes() -> Router<AppState> {
    Router::new()
        .typed_get(session::create)
        .typed_post(session::store)
        .layer(from_fn(middleware::redirect_if_authenticated::handle))
}

fn authenticated_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .typed_get(dashboard::show)
        .layer(from_fn_with_state(
            state.clone(),
            middleware::authenticate::handle,
        ))
}
