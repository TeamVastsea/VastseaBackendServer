mod config;

use std::fs::{OpenOptions};
use std::io::{Read};
use chrono::{Local};
use mongodb::Client;
use mongodb::options::ClientOptions;
use simple_log::{info, LogConfigBuilder};

#[tokio::main]
async fn main() {
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
    {
        let mut config: String = Default::default();
        let mut file = OpenOptions::new().read(true).write(true).create(true).open("server.config.json").expect("Can not open 'server.config.json'");
        file.read_to_string(&mut config).expect("Can not read 'server.config.json'");
        if config == "" {
            config = "{}".to_string();
        }
        server_config = serde_json::from_str(config.as_str()).expect("Can not parse 'server.config.json' as json");
    }
    config::init(&mut server_config);
}
