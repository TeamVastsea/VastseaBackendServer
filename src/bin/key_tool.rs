use std::fs::OpenOptions;
use std::io;
use std::io::{Read, Write};
use std::process::exit;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use bson::doc;
use chrono::{DateTime, Months, Utc};
use futures::executor::block_on;
use futures_util::stream::StreamExt;
use jwt_simple::prelude::HS256Key;
use lazy_static::lazy_static;
use mongodb::{Client, Collection, Cursor, Database};
use mongodb::options::ClientOptions;
use prettytable::{Cell, row, Row, Table};
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiKey {
    _id: String,
    usage: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub nbf: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub nat: DateTime<Utc>,

}


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
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionSetting {
    #[serde_inline_default(false)]
    pub tls: bool,
    #[serde_inline_default(7890)]
    pub server_port: u16,
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


fn generate_default_user_group() -> Vec<String> {
    vec!["default".to_string()]
}

fn generate_default_key() -> String {
    STANDARD.encode(HS256Key::generate().to_bytes())
}

fn generate_connect_setting() -> ConnectionSetting {
    ConnectionSetting {
        tls: false,
        server_port: 7890,
        ssl_cert: "./cert.crt".to_string(),
        ssl_key: "./private.key".to_string(),
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

pub fn init_config() -> ServerConfig {
    println!("Loading configs...");
    let mut raw_config = String::new();
    let mut file = OpenOptions::new().read(true).write(true).create(true).open("config.toml").expect("Cannot open 'config.toml'");
    file.read_to_string(&mut raw_config).unwrap();

    let config: ServerConfig = toml::from_str(&raw_config).unwrap();


    if toml::to_string_pretty(&config).unwrap() != raw_config {
        save_config(&config)
    }

    config
}

pub fn save_config(config: &ServerConfig) {
    println!("Config changed, please edit and restart");
    let config_str = toml::to_string_pretty(config).unwrap();

    let mut file = OpenOptions::new().write(true).truncate(true).open("config.toml").expect("Can not open 'config.toml'");
    file.write(config_str.as_bytes()).unwrap();

    panic!("config changed");
}

pub fn get_mongodb() -> Database {
    let mongo_options = block_on(ClientOptions::parse(&CONFIG.mongodb.uri)).unwrap();

    let client = Client::with_options(mongo_options).expect("Can not connect to mongodb");
    client.database(&CONFIG.mongodb.db_name)
}


lazy_static! {
    static ref CONFIG: ServerConfig = init_config();
    static ref MONGODB: Database = get_mongodb();
}


#[tokio::main]
async fn main() {
    let db_url = &CONFIG.mongodb.uri;
    let db_name = &CONFIG.mongodb.db_name;

    println!("db url: {}\ndb name: {}", db_url, db_name);

    let mongo_options: ClientOptions = ClientOptions::parse(db_url).await.expect("Can not connect to mongodb");
    let client = Client::with_options(mongo_options).expect("Can not connect to mongodb");
    let db = client.database(db_name);

    let mut buffer = "".to_string();
    let stdin = io::stdin();

    loop {
        print!("\x1b[2J");
        print!("\x1b[H");
        println!("Please select the options below:\n1) Create API key\n2) Check API key\n3) Delete API key\n4) List all available key(s)\n5) Quit(default) ");
        stdin.read_line(&mut buffer).expect("Cannot read from stdin...");
        let input;

        if buffer.chars().nth(0).is_none() {
            input = "4".to_string();
        } else {
            input = buffer.chars().nth(0).unwrap().to_string().trim_end().to_string();
        }
        buffer.clear();

        match input.as_str() {
            "1" => {
                let key = STANDARD.encode(HS256Key::generate().to_bytes()).replace("+", "-").replace("/", "*").replace("=", "~");
                println!("Please input the usage of this key");
                stdin.read_line(&mut buffer).expect("Cannot read from stdin...");
                let input = buffer.trim_end().to_string();
                buffer.clear();

                let nbf = bson::DateTime::now().to_chrono();
                let nat = nbf.checked_add_months(Months::new(3)).unwrap();

                let collection: &Collection<ApiKey> = &db.collection("api_key");

                let document = ApiKey {
                    _id: key,
                    usage: input,
                    nbf,
                    nat,
                };

                collection.insert_one(document.clone(), None).await.expect("Cannot write mongodb.");
                println!("Create succeeded!\nYour key is {}\nValid before {}.", document._id, document.nat.format("%Y-%m-%d %H:%M:%S"));
                pause();
            }
            "2" => {
                println!("Please input the key");
                stdin.read_line(&mut buffer).expect("Cannot read from stdin...");
                let input = buffer.trim_end().to_string();
                buffer.clear();

                let collection: &Collection<ApiKey> = &db.collection("api_key");

                let doc = collection.find_one(doc! {"_id": input}, None).await.expect("Cannot queue mongodb.");
                if doc.is_none() {
                    println!("Cannot find your key in database.");
                    continue;
                }
                let doc = doc.unwrap();

                println!("The info of the key:\nYour key is for {}\nValid after {}\nValid before {}.", doc.usage, doc.nbf.format("%Y-%m-%d %H:%M:%S"), doc.nat.format("%Y-%m-%d %H:%M:%S"));
                pause();
            }
            "3" => {
                println!("Please input the key");
                stdin.read_line(&mut buffer).expect("Cannot read from stdin...");
                let input = buffer.trim_end().to_string();
                buffer.clear();

                let collection: &Collection<ApiKey> = &db.collection("api_key");

                let result: Cursor<ApiKey> = collection.find(doc! {"_id": input.clone()}, None).await.expect("Cannot find mongodb");
                let counts: Vec<Result<ApiKey, _>> = result.collect().await;
                println!("Will affect {} item(s)", counts.len());
                println!("Are you sure to continue?(y/N)");

                stdin.read_line(&mut buffer).expect("Cannot read from stdin...");
                let input2 = buffer.trim_end().to_string();
                buffer.clear();

                if input2 == "y" || input2 == "Y" {
                    let result = collection.delete_many(doc! {"_id": input}, None).await.expect("Cannot delete mongodb");
                    println!("Success! {} item(s) affected.", result.deleted_count);
                }
                pause();
            }
            "4" => {
                let collection: &Collection<ApiKey> = &db.collection("api_key");

                let result: Cursor<ApiKey> = collection.find(doc! {}, None).await.expect("Cannot find mongodb");
                let keys: Vec<Result<ApiKey, _>> = result.collect().await;

                let mut table = Table::new();
                table.add_row(row!["KEY", "FOR", "NOT BEFORE", "NOT AFTER"]);

                for key in keys {
                    if let Ok(key_info) = key {
                        if bson::DateTime::now() < key_info.nbf.into() || bson::DateTime::now() > key_info.nat.into() {
                            table.add_row(Row::new(vec![
                                Cell::new(key_info._id.as_str()).style_spec("Fr"),
                                Cell::new(key_info.usage.as_str()),
                                Cell::new(key_info.nbf.format("%Y-%m-%d %H:%M:%S").to_string().as_str()),
                                Cell::new(key_info.nat.format("%Y-%m-%d %H:%M:%S").to_string().as_str()),
                            ]));
                        } else {
                            table.add_row(row![key_info._id, key_info.usage, key_info.nbf.format("%Y-%m-%d %H:%M:%S"), key_info.nat.format("%Y-%m-%d %H:%M:%S")]);
                        }
                    }
                }

                table.printstd();

                println!("Keys in red are out dated.");
                pause();
            }
            _ => { exit(0); }
        }
    }
}

fn pause() {
    println!("Press any key to continue...");
    let stdin = io::stdin();
    let mut _buffer = "".to_string();
    stdin.read_line(&mut _buffer).expect("Cannot read from stdin...");
}