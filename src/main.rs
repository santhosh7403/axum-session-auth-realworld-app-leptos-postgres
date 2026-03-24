#![recursion_limit = "256"]
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use axum_session_auth_realworld_app_leptos_postgres::app::*;
    use axum_session_auth_realworld_app_leptos_postgres::database;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    tracing_subscriber::fmt()
        .with_level(true)
        .with_max_level(tracing::Level::DEBUG)
        .init();
    // Init the pool into static
    database::init_db()
        .await
        .expect("problem during initialization of the database");

    let pool = database::get_db().clone();

    // Session Layer Config
    let session_config = axum_session::SessionConfig::default()
        .with_db_update_interval(chrono::Duration::try_seconds(30).unwrap_or_default())
        .with_table_name("sessions_table");

    let session_store = axum_session::SessionStore::<axum_session_sqlx::SessionPgPool>::new(
        Some(pool.clone().into()),
        session_config,
    )
    .await
    .unwrap();

    let session_layer = axum_session::SessionLayer::new(session_store);

    // Auth Layer Config
    let auth_config = axum_session_auth::AuthConfig::<String>::default();
    let auth_layer = axum_session_auth::AuthSessionLayer::<
        axum_session_auth_realworld_app_leptos_postgres::models::User,
        String,
        axum_session_sqlx::SessionPgPool,
        sqlx::PgPool,
    >::new(Some(pool))
    .with_config(auth_config);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(
                    tower_http::trace::DefaultMakeSpan::new().level(tracing::Level::DEBUG),
                )
                .on_request(tower_http::trace::DefaultOnRequest::new().level(tracing::Level::DEBUG))
                .on_response(
                    tower_http::trace::DefaultOnResponse::new().level(tracing::Level::DEBUG),
                )
                .on_failure(
                    tower_http::trace::DefaultOnFailure::new().level(tracing::Level::DEBUG),
                ),
        )
        .layer(auth_layer)
        .layer(session_layer)
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
