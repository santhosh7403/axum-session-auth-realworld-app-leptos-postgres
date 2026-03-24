// use axum::{
//     http::{header, Request, StatusCode},
//     response::Response,
// };
use leptos::prelude::*;

#[cfg(feature = "ssr")]
pub type AuthSession = axum_session_auth::AuthSession<
    crate::models::User,
    String,
    axum_session_sqlx::SessionPgPool,
    sqlx::PgPool,
>;

#[tracing::instrument]
#[cfg(feature = "ssr")]
pub fn get_username() -> Option<String> {
    if let Some(req) = use_context::<axum::http::request::Parts>() {
        req.extensions
            .get::<AuthSession>()
            .and_then(|auth| auth.current_user.clone().map(|u| u.username()))
    } else {
        None
    }
}
