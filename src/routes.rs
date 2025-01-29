use crate::{
    features::{dashboard, session},
    middleware,
    // features::{authenticated, layout::wrap_in_layout},
    state::AppState,
};
use axum::{
    middleware::{from_fn, from_fn_with_state},
    routing::{get, post},
    Router,
};
use tower_cookies::CookieManagerLayer;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(public_routes())
        .merge(authenticated_routes(state))
        .layer(CookieManagerLayer::new())
}

fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/ping", get(|| async { "pong" }))
        .merge(
            Router::new()
                .route("/login", get(session::create))
                .route("/sessions/create", post(session::store))
                .layer(from_fn(middleware::redirect_if_authenticated::handle)),
        )
}

fn authenticated_routes(state: AppState) -> Router<AppState> {
    Router::new()
        // .typed_get(authenticated::pages::show::show)
        .route("/dashboard", get(dashboard::show))
        .layer(from_fn_with_state(
            state.clone(),
            middleware::authenticate::handle,
        ))
}
