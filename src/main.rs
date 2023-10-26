mod config;
mod user;
mod api;
mod survey;
mod news;
mod github;
mod utils;

use std::fs;
use std::io::BufReader;
use axum::{Json, Router, ServiceExt};
use axum::routing::{get, patch, post, put};
use axum_server::tls_rustls::RustlsConfig;
use chrono::{Local};
use lazy_static::lazy_static;
use mongodb::Database;
use mongodb::bson::doc;
use serde_json::{json, Value};
use shadow_rs::shadow;
use simple_log::{debug, info, LogConfigBuilder};
use tower_http::catch_panic::CatchPanicLayer;
use crate::api::{user_ban_put, user_bind_qq_patch, user_luck_get, user_qq_get};
use crate::config::ServerConfig;
use crate::github::github_post_receive;
use crate::news::{news_get, news_id_get};
use crate::user::user_get;

lazy_static! {
    static ref CONFIG: ServerConfig = config::get_log();
    static ref MONGODB: Database = config::get_mongodb();
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut file_name = "./log/".to_owned();
    file_name += &Local::now().format("%Y-%m-%d.%H-%M-%S").to_string();
    file_name += ".log";

    let config = LogConfigBuilder::builder()
        .path(&file_name)
        .size(1 * 100)
        .roll_count(10)
        .time_format("%Y-%m-%d %H:%M:%S.%f") //E.g:%H:%M:%S.%f
        .level("debug")
        .output_file()
        .output_console()
        .build();
    simple_log::new(config).expect("Cannot init logger");

    let app = Router::new()
        .route("/user", get(user_get))
        .route("/user", put(user_ban_put))
        .route("/user", patch(user_bind_qq_patch))
        .route("/user/qq", get(user_qq_get))
        .route("/user/luck", get(user_luck_get))
        .route("/news", get(news_get))
        .route("/news/:id", get(news_id_get))
        .route("/github", post(github_post_receive))
        .route("/panic", get(panic))
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

fn load_certs(filename: &str) -> Vec<rustls::Certificate> {
    let certfile = fs::File::open(filename).expect("cannot open certificate file");
    let mut reader = BufReader::new(certfile);
    rustls_pemfile::certs(&mut reader)
        .unwrap()
        .iter()
        .map(|v| rustls::Certificate(v.clone()))
        .collect()
}

fn load_private_key(filename: &str) -> rustls::PrivateKey {
    let keyfile = fs::File::open(filename).expect("cannot open private key file");
    let mut reader = BufReader::new(keyfile);

    loop {
        match rustls_pemfile::read_one(&mut reader).expect("cannot parse private key .pem file") {
            Some(rustls_pemfile::Item::RSAKey(key)) => return rustls::PrivateKey(key),
            Some(rustls_pemfile::Item::PKCS8Key(key)) => return rustls::PrivateKey(key),
            Some(rustls_pemfile::Item::ECKey(key)) => return rustls::PrivateKey(key),
            None => break,
            _ => {}
        }
    }

    panic!(
        "no keys found in {:?} (encrypted keys are not supported)",
        filename
    );
}

async fn ping() -> Json<Value> {
    shadow!(build);
    Json(json! ({"version": 2, "build_time": build::BUILD_TIME, "commit": build::SHORT_COMMIT, "rust_version": build::RUST_VERSION}))
}

async fn panic() {
    let a: Option<i32> = None;
    let b = a.unwrap();
}