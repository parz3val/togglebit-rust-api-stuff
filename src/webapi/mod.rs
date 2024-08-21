use crate::darkscout::adapters::dsbreach::BreachClient;
use crate::darkscout::adapters::smtp_mailer::create_smtp_client;
use crate::darkscout::adapters::ds_darkengine::api::DarkSearchClient;
use crate::darkscout::adapters::{DSProvider, DarkSearchProvider};
use crate::darkscout::types::store::DSCache;
use crate::darkscout::utils::middlewares::jwt_auth::authorization_middleware;
use crate::darkscout::{
    handlers,
    types::{
        store::{NewDb, PgStore},
        AppState, Settings, SettingsEnv,
    },
};

use axum::middleware;
use axum::routing::{get, post};
use http::header::{ACCEPT, ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, CONTENT_TYPE};
use http::Method;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

async fn test() -> &'static str {
    "Hello World"
}
pub async fn web_api() -> axum::Router {
    let app_env = SettingsEnv::from_file().await;
    let settings: Settings = app_env.config.clone();
    // TODO: Do not leak string into static
    // change the new function in HIBClient
    let my_static_key: &'static str = Box::leak(settings.clone().dsbreach_api_key.into_boxed_str());
    let cache = DSCache::new(&settings).await;
    let ds_provider = match BreachClient::new(my_static_key, cache.clone().l3).await {
        Ok(client) => client,
        Err(e) => {
            tracing::debug!("Failed to create dsbreach client: {}", e);
            panic!("Couldn't create dsbreach client")
        }
    };

    let ds_darkengine_provider =
        DarkSearchClient::new(settings.ds_darkengine_api_key.as_str(), cache.clone().l3);

    let smtp_client =
        create_smtp_client(app_env.smtp.username.clone(), app_env.smtp.password.clone());
    let app_state: AppState = AppState {
        settings: app_env,
        db: Arc::new(PgStore::new(&settings).await), // new will panic if can't be connected to db
        cache: Arc::new(cache),
        ds_provider,
        smtp_client,
        ds_darkengine_provider,
    };

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([
            AUTHORIZATION,
            ACCEPT,
            CONTENT_TYPE,
            ACCESS_CONTROL_ALLOW_ORIGIN,
        ])
        // allow requests from any origin
        .allow_origin(Any);

    axum::Router::new()
        // Unauthorized Routes
        .route("/", get(test))
        .route("/users/auth/signup", post(handlers::users::sign_up))
        .route("/users/auth/login", post(handlers::users::log_in))
        .route("/users/auth/verify/", post(handlers::users::verify_user))
        .route(
            "/users/auth/request-reset-token/",
            post(handlers::auth::request_password_reset_link),
        )
        .route(
            "/users/auth/reset-with-token/",
            post(handlers::auth::reset_password_with_token),
        )
        // Authorized Routes
        // Auth
        .route(
            "/users/auth/reset-password/",
            post(handlers::users::change_password)
                .layer(middleware::from_fn(authorization_middleware)),
        )
        // Users
        .route(
            "/users/edit/profile",
            post(handlers::users::edit_user_profile)
                .layer(middleware::from_fn(authorization_middleware)),
        )
        // dark monitor
        .route(
            "/dark-monitor/email/:email",
            get(handlers::dark_monitor::get_stats_by_email)
                .layer(middleware::from_fn(authorization_middleware)),
        )
        .route(
            "/dark-monitor/analytics/email/:email",
            get(handlers::dark_monitor::get_analytics_by_email)
                .layer(middleware::from_fn(authorization_middleware)),
        )
        .route(
            "/dark-monitor/domain/:domain",
            get(handlers::dark_monitor::get_stats_by_domain)
                .layer(middleware::from_fn(authorization_middleware)),
        )
        // DARK SEARCH
        .route(
            "/dark-search/domain/:domain",
            get(handlers::dark_monitor::get_dark_search_by_domain)
                .layer(middleware::from_fn(authorization_middleware)),
        )
        .route(
            "/dark-search/email/:email",
            get(handlers::dark_monitor::get_dark_search_by_email)
                .layer(middleware::from_fn(authorization_middleware)),
        )
        // workspace
        // workspace
        .route(
            "/workspace/create/",
            get(handlers::workspaces::create_workspace)
                .layer(middleware::from_fn(authorization_middleware)),
        )
        .route(
            "/workspace/members/invite",
            get(handlers::workspaces::invite_new_members)
                .layer(middleware::from_fn(authorization_middleware)),
        )
        // add verified emails
        .route(
            "/workspace/verified-emails",
            post(handlers::verified_emails::add_verified_emails)
                .layer(middleware::from_fn(authorization_middleware)),
        )
        // add verified domains
        .route(
            "/workspace/verified-domains",
            post(handlers::verified_domains::add_verified_domains)
                .layer(middleware::from_fn(authorization_middleware)),
        )
        .route(
            "/workspace/edit/{workspace_id}",
            post(handlers::workspaces::edit_workspace)
                .layer(middleware::from_fn(authorization_middleware)),
        )
        // member settings
        .layer(cors)
        .route(
            "/settings/{member_id}", // don't need workspace id because its added from login
            get(handlers::members::get_member_settings)
                .layer(middleware::from_fn(authorization_middleware)),
        )
        .with_state(app_state)
        .layer(TraceLayer::new_for_http())
}
