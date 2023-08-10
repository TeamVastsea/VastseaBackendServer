mod config;
mod user;
mod api;
mod survey;
mod utils;
mod news;

use std::fs;
use std::io::BufReader;
use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use chrono::{Local};
use lazy_static::lazy_static;
use mongodb::Database;
use mongodb::bson::doc;
use shadow_rs::shadow;
use simple_log::{info, LogConfigBuilder};
use crate::config::ServerConfig;


// static mut CONFIG: Value = Value::Null;
// static mut MONGODB: Option<Database> = None;

lazy_static! {
    static ref CONFIG: ServerConfig = config::init();
    static ref MONGODB: Database = config::get_mongodb();
}

#[actix_web::main]
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

    //start server
    let server = HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::new("%a %r -> %s with in %Dms, %bb"))
            .service(ping)
            .service(user::user_get)
            .service(news::news_get)
            .service(news::news_details)
            .service(news::news_create)
            .service(api::user_patch)
            .service(api::user_put)
            .service(api::user_qq_get)
            .service(api::user_luck_get)
    });

    let tls = CONFIG.connection.tls;
    if !tls {
        info!("Listening: http://0.0.0.0:{}", CONFIG.connection.server_port);
        server.bind(("0.0.0.0", CONFIG.connection.server_port)).expect("Can not bind server to port").run().await.expect("Can not start server");
    } else {
        info!("Loading certs...");
        let certs = load_certs(&CONFIG.connection.ssl_cert);
        let private_key = load_private_key(&CONFIG.connection.ssl_cert);

        let config = rustls::ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certs, private_key)
            .expect("bad certificate/key");

        info!("Listening: https://0.0.0.0:{}", CONFIG.connection.server_port);
        server.bind_rustls(("0.0.0.0", CONFIG.connection.server_port), config).expect("Can not bind server to port").run().await.expect("Can not start server");
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
        "no keys found in {:?} (encrypted keys not supported)",
        filename
    );
}

#[get("/ping")]
async fn ping() -> impl Responder {
    shadow!(build);
    HttpResponse::Ok().body(doc! {"version": 2, "build_time": build::BUILD_TIME, "commit": build::SHORT_COMMIT, "rust_version": build::RUST_VERSION}.to_string())
}