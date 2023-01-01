mod config;
mod user;

use std::fs::{OpenOptions};
use std::io::{Read};
use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use chrono::{Local};
use mongodb::{Client, Database};
use mongodb::options::ClientOptions;
use serde_json::Value;
use simple_log::{debug, info, LogConfigBuilder, warn};


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

    user::register::register("test01".to_string(), "123456".to_string(), "test@1.com".to_string()).await.unwrap();

    HttpServer::new(|| {
        App::new()
            .service(echo)
            .service(user::login_request)
    }).bind(("0.0.0.0", unsafe { &CONFIG }["connection"]["serverPort"].as_i64().unwrap() as u16)).expect("Can not bind server to port").run().await.expect("Can not start server");
    Ok(())
}


#[get("/ping")]
async fn echo(_req_body: String) -> impl Responder {
    HttpResponse::Ok().body("api v2")
}