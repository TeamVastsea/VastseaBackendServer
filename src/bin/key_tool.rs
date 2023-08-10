use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use std::process::exit;
use base64::Engine;
use bson::doc;
use chrono::{DateTime, Months, Utc};
use jwt_simple::prelude::HS256Key;
use mongodb::{Client, Collection, Cursor};
use mongodb::options::ClientOptions;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use futures_util::stream::StreamExt;
use prettytable::{Cell, row, Row, Table};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiKey {
    _id: String,
    usage: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub nbf: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub nat: DateTime<Utc>,

}

#[tokio::main]
async fn main() {
    println!("Loading...");
    let server_config: Value;
    let mut config: String = Default::default();
    let mut file = OpenOptions::new().read(true).write(true).create(true).open("server.config.json").expect("Can not open 'server.config.json'");
    file.read_to_string(&mut config).expect("Can not read 'server.config.json'");
    if config == "" {
        config = "{}".to_string();
    }
    server_config = serde_json::from_str(config.as_str()).expect("Can not parse 'server.config.json' as json");
    let db_url = server_config["mongodb"]["dbUrl"].as_str().expect("Missing dbUrl");
    let db_name = server_config["mongodb"]["dbName"].as_str().expect("Missing dbName");

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
                let key = base64::engine::general_purpose::STANDARD.encode(HS256Key::generate().to_bytes()).replace("+", "-").replace("/", "*").replace("=", "~");
                println!("Please input the usage of this key");
                stdin.read_line(&mut buffer).expect("Cannot read from stdin...");
                let input = buffer.trim_end().to_string();
                buffer.clear();

                let nbf = bson::DateTime::now().to_chrono();
                let nat = nbf.checked_add_months(Months::new(3)).unwrap();

                let collection: &Collection<ApiKey> = &db.collection("api_key");

                let document = ApiKey{
                    _id: key,
                    usage: input,
                    nbf,
                    nat,
                };

                collection.insert_one(document.clone(), None).await.expect("Cannot write mongodb.");
                println!("Create succeeded!\nYour key is {}\nValid before {}.", document._id, document.nat.format("%Y-%m-%d %H:%M:%S"));
                pause();
            },
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
            },
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
            },
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
                                Cell::new(key_info.nat.format("%Y-%m-%d %H:%M:%S").to_string().as_str())
                            ]));
                        } else {
                            table.add_row(row![key_info._id, key_info.usage, key_info.nbf.format("%Y-%m-%d %H:%M:%S"), key_info.nat.format("%Y-%m-%d %H:%M:%S")]);
                        }
                    }
                }

                table.printstd();

                println!("Keys in red are out dated.");
                pause();
            },
            _ => { exit(0); },
        }
    }
}

fn pause() {
    println!("Press any key to continue...");
    let stdin = io::stdin();
    let mut _buffer = "".to_string();
    stdin.read_line(&mut _buffer).expect("Cannot read from stdin...");
}