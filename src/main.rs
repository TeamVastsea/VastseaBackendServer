use axum::{Json, Router};
use axum::routing::{get, patch, post, put};
use axum_server::tls_rustls::RustlsConfig;
use lazy_static::lazy_static;
use mongodb::bson::doc;
use mongodb::Database;
use serde_json::{json, Value};
use shadow_rs::shadow;
use tower_http::catch_panic::CatchPanicLayer;
use tracing::{debug, info};
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{EnvFilter, fmt, Registry};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::api::{user_ban_put, user_bind_qq_patch, user_luck_get, user_qq_get};
use crate::config::ServerConfig;
use crate::github::github_post_receive;
use crate::news::{news_get, news_id_get};
use crate::user::user_get;

mod config;
mod user;
mod api;
mod survey;
mod news;
mod github;
mod utils;

lazy_static! {
    static ref CONFIG: ServerConfig = config::get_log();
    static ref MONGODB: Database = config::get_mongodb();
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));

    let formatting_layer = fmt::layer().with_writer(std::io::stderr);
    let file_appender = rolling::daily("log", "log");
    let (non_blocking_appender, _guard) = non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking_appender);
    Registry::default()
        .with(env_filter)
        .with(formatting_layer)
        .with(file_layer)
        .init();

    let app = Router::new()
        .route("/ping", get(ping))
        .route("/user", get(user_get))
        .route("/user", put(user_ban_put))
        .route("/user", patch(user_bind_qq_patch))
        .route("/user/qq", get(user_qq_get))
        .route("/user/luck", get(user_luck_get))
        .route("/news", get(news_get))
        .route("/news/:id", get(news_id_get))
        .route("/github", post(github_post_receive))
        .layer(CatchPanicLayer::new());

    let addr = CONFIG.connection.server_url.parse().unwrap();
    info!("Listening: {addr}");

    if CONFIG.connection.tls {
        info!("HTTPS enabled.");
        let tls_config = RustlsConfig::from_pem_file(CONFIG.connection.ssl_cert.clone(), CONFIG.connection.ssl_key.clone()).await.unwrap();
        axum_server::bind_rustls(addr, tls_config).serve(app.into_make_service()).await.unwrap();
    } else {
        debug!("HTTPS disabled.");
        axum_server::bind(addr).serve(app.into_make_service()).await.unwrap();
    }
    Ok(())
}

async fn ping() -> Json<Value> {
    shadow!(build);
    Json(json!({"version": 2, "build_time": build::BUILD_TIME, "commit": build::SHORT_COMMIT, "rust_version": build::RUST_VERSION}))
}