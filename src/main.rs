mod config;
mod user;

use std::fs;
use std::fs::{OpenOptions};
use std::io::{BufReader, Read};
use actix_web::{App, HttpResponse, HttpServer, Responder, get, HttpRequest};
use chrono::{Local};
use mongodb::{Client, Database};
use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use serde_json::Value;
use shadow_rs::shadow;
use simple_log::{info, LogConfigBuilder};
use simple_log::log::log;


static mut CONFIG: Value = Value::Null;
static mut MONGODB: Option<Database> = None;

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

    info!("Initializing...");

    //read server.config.json
    let mut server_config;
    let mut config: String = Default::default();
    let mut file = OpenOptions::new().read(true).write(true).create(true).open("server.config.json").expect("Can not open 'server.config.json'");
    file.read_to_string(&mut config).expect("Can not read 'server.config.json'");
    if config == "" {
        config = "{}".to_string();
    }
    server_config = serde_json::from_str(config.as_str()).expect("Can not parse 'server.config.json' as json");
    config::init(&mut server_config);
    unsafe { CONFIG = server_config; }


    //init mongodb
    let mongo_options: ClientOptions = ClientOptions::parse(&unsafe { &CONFIG }["mongodb"]["dbUrl"].as_str().unwrap()).await.expect("Can not connect to mongodb");
    let client = Client::with_options(mongo_options).expect("Can not connect to mongodb");
    let db = client.database(&unsafe { &CONFIG }["mongodb"]["dbName"].as_str().unwrap());
    unsafe { MONGODB = Some(db); }

    let tls = unsafe { &CONFIG }["connection"]["tls"].as_bool().unwrap();

    let server = HttpServer::new(|| {
        App::new()
            .service(ping)
            .service(user::user_get)
    });

    if !tls {
        server.bind(("0.0.0.0", unsafe { &CONFIG }["connection"]["serverPort"].as_u64().unwrap() as u16)).expect("Can not bind server to port").run().await.expect("Can not start server");
    } else {
        info!("Loading certs...");
        let certs = load_certs(unsafe { &CONFIG }["connection"]["sslCert"].as_str().unwrap());
        let private_key = load_private_key(unsafe { &CONFIG }["connection"]["sslKey"].as_str().unwrap());

        let config = rustls::ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certs, private_key)
            .expect("bad certificate/key");
        server.bind_rustls(("0.0.0.0", unsafe { &CONFIG }["connection"]["serverPort"].as_u64().unwrap() as u16), config).expect("Can not bind server to port").run().await.expect("Can not start server");
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
async fn ping(req: HttpRequest) -> impl Responder {
    shadow!(build);
    info!("200/ping->{}: {}", req.peer_addr().unwrap().ip().to_string(), doc! {"version": 2, "build_time": build::BUILD_TIME, "commit": build::SHORT_COMMIT, "rust_version": build::RUST_VERSION}.to_string());
    HttpResponse::Ok().body(doc! {"version": 2, "build_time": build::BUILD_TIME, "commit": build::SHORT_COMMIT, "rust_version": build::RUST_VERSION}.to_string())
}