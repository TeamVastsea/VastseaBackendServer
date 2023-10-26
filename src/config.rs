use std::fs::OpenOptions;
use std::io::{Read, Write};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use futures::executor::block_on;
use jwt_simple::prelude::HS256Key;
use mongodb::{Client, Database};
use mongodb::options::ClientOptions;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

use simple_log::{error, info};
use crate::CONFIG;

#[serde_inline_default]
#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    #[serde(default = "generate_default_user_group")]
    pub default_user_group: Vec<String>,
    #[serde(default = "generate_default_key")]
    pub token_key: String,
    #[serde(default = "generate_connect_setting")]
    pub connection: ConnectionSetting,
    #[serde(default = "generate_mongodb_setting")]
    pub mongodb: MongodbSetting,
    #[serde(default = "generate_rabbitmq_setting")]
    pub rabbitmq: RabbitmqSetting,
    #[serde(default = "generate_oauth_setting")]
    pub oauth: OAuthSetting,
    #[serde(default = "generate_luck_gen_setting")]
    pub luck: LuckGenSetting,
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionSetting {
    #[serde_inline_default(false)]
    pub tls: bool,
    #[serde_inline_default(String::from("0.0.0.0:7890"))]
    pub server_url: String,
    #[serde_inline_default(String::from("./cert.crt"))]
    pub ssl_cert: String,
    #[serde_inline_default(String::from("./private.key"))]
    pub ssl_key: String,
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Debug)]
pub struct MongodbSetting {
    #[serde_inline_default(String::from("mongodb://127.0.0.1:27017"))]
    pub uri: String,
    #[serde_inline_default(String::from("backend"))]
    pub db_name: String,
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Debug)]
pub struct RabbitmqSetting {
    #[serde_inline_default(String::from("amqp://guest:guest@localhost:5672"))]
    pub uri: String,
    #[serde_inline_default(String::from("amqprs.example"))]
    pub rounting_key: String,
    #[serde_inline_default(String::from("amq.topic"))]
    pub exchange_name: String,
    #[serde_inline_default(String::from("github"))]
    pub queue_name: String,
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Debug)]
pub struct OAuthSetting {
    #[serde_inline_default(String::from("646004c1-0054-4157-b5b5-4cb89f6eaa1a"))]
    pub client_id: String,
    #[serde_inline_default(String::from("https://mccteam.github.io/redirect.html"))]
    pub redirect_url: String,
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Debug)]
pub struct LuckGenSetting {
    #[serde_inline_default(1)]
    pub m: u64,
    #[serde_inline_default(2)]
    pub a: u64,
    #[serde_inline_default(3)]
    pub c: u64,
}

fn generate_default_user_group() -> Vec<String> {
    vec!["default".to_string()]
}

fn generate_default_key() -> String {
    STANDARD.encode(HS256Key::generate().to_bytes())
}

fn generate_connect_setting() -> ConnectionSetting {
    ConnectionSetting {
        tls: false,
        server_url: "0.0.0.0:7890".to_string(),
        ssl_cert: "./cert.crt".to_string(),
        ssl_key: "./private.key".to_string(),
    }
}

fn generate_oauth_setting() -> OAuthSetting {
    OAuthSetting {
        client_id: "646004c1-0054-4157-b5b5-4cb89f6eaa1a".to_string(),
        redirect_url: "https://mccteam.github.io/redirect.html".to_string(),
    }
}

fn generate_mongodb_setting() -> MongodbSetting {
    MongodbSetting {
        uri: "mongodb://127.0.0.1:27017".to_string(),
        db_name: "backend".to_string(),
    }
}

fn generate_rabbitmq_setting() -> RabbitmqSetting {
    RabbitmqSetting {
        uri: "amqp://guest:guest@localhost:5672".to_string(),
        rounting_key: "amqprs.example".to_string(),
        exchange_name: "amq.topic".to_string(),
        queue_name: "github".to_string(),
    }
}

fn generate_luck_gen_setting() -> LuckGenSetting {
    LuckGenSetting {
        m: 1,
        a: 2,
        c: 3,
    }
}

pub fn get_log() -> ServerConfig {
    info!("Loading configs...");
    let mut raw_config = String::new();
    let mut file = OpenOptions::new().read(true).write(true).create(true).open("config.toml").expect("Cannot open 'config.toml'");
    file.read_to_string(&mut raw_config).unwrap();

    let config: ServerConfig = toml::from_str(&raw_config).unwrap();


    if toml::to_string_pretty(&config).unwrap() != raw_config {
        save(&config)
    }

    config
}

pub fn save(config: &ServerConfig) {
    error!("Config changed, please edit and restart");
    let config_str = toml::to_string_pretty(config).unwrap();

    let mut file = OpenOptions::new().write(true).truncate(true).open("config.toml").expect("Cannot open 'config.toml'");
    file.write(config_str.as_bytes()).unwrap();

    panic!("config changed");
}

pub fn get_mongodb() -> Database {
    let mongo_options = block_on(ClientOptions::parse(&CONFIG.mongodb.uri)).unwrap();

    let client = Client::with_options(mongo_options).expect("Can not connect to mongodb");
    client.database(&CONFIG.mongodb.db_name)
}