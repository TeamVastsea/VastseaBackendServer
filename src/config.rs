use std::fs::OpenOptions;
use std::io::Write;
use base64::Engine;
use jwt_simple::prelude::HS256Key;

use serde_json::Value;
use serde_json::Value::Null;
use simple_log::{info, warn};

pub fn init(config: &mut Value) {
    info!("Checking configs...");
    let mut edited: bool = false;
    // let mut rng = rand::thread_rng();

    if config["allowLocalHost"] == Null {
        warn!("'allowLocalHost' not found, setting to  false  by default.");
        config["allowLocalHost"] = Value::from(false);
        edited = true;
    }
    if config["baseUrl"] == Null {
        warn!("'baseUrl' not found, setting to  '127.0.0.1'  by default.");
        config["baseUrl"] = Value::from("127.0.0.1");
        edited = true;
    }
    if config["defaultUserGroup"] == Null {
        warn!("'defaultUserGroup' not found, setting to  'default'  by default.");
        config["defaultUserGroup"] = Value::from("default");
        edited = true;
    }
    // if config["pluginPassword"] == Null {
    //     let pass: String = Alphanumeric.sample_iter(&mut rng).take(16).map(char::from).collect::<String>();
    //     warn!("'pluginPassword' not found, setting to  '{}'  by default.", pass);
    //     config["pluginPassword"] = Value::from(pass);
    //     edited = true;
    // }
    if config["tokenKey"] == Null {
        let key = HS256Key::generate();
        warn!("'tokenKey' not found, setting to  '{}' .", base64::engine::general_purpose::STANDARD.encode(key.to_bytes()));
        config["tokenKey"] = Value::from(base64::engine::general_purpose::STANDARD.encode(key.to_bytes()));
        edited = true;
    }


    if config["connection"]["serverPort"] == Null {
        warn!("'connection.serverPort' not found, setting to  7890  by default.");
        config["connection"]["serverPort"] = Value::from(7890);
        edited = true;
    }
    if config["connection"]["sslCert"] == Null {
        warn!("'connection.sslCert' not found, setting to  '.\\keys\\my_domain.crt'  by default.");
        config["connection"]["sslCert"] = Value::from(".\\keys\\my_domain.crt");
        edited = true;
    }
    if config["connection"]["sslKey"] == Null {
        warn!("'connection.sslKey' not found, setting to  '.\\keys\\private.key'  by default.");
        config["connection"]["sslKey"] = Value::from(".\\keys\\private.key");
        edited = true;
    }


    if config["main"]["enabled"] == Null {
        warn!("'mail.enabled' not found, setting to  false  by default.");
        config["main"]["enabled"] = Value::from(false);
        edited = true;
    }
    if config["mail"]["secureConnection"] == Null {
        warn!("'mail.secureConnection' not found, setting to  false  by default.[Mail will be disabled! Please enable after change!]");
        config["mail"]["secureConnection"] = Value::from(false);
        config["main"]["enabled"] = Value::from(false);
        edited = true;
    }
    if config["mail"]["smtpUrl"] == Null {
        warn!("'mail.smtpHost' not found, setting to  'smtp.126.com:25'  by default.[Mail will be disabled! Please enable after change!]");
        config["mail"]["smtpUrl"] = Value::from("smtp.126.com:25");
        config["main"]["enabled"] = Value::from(false);
        edited = true;
    }
    if config["mail"]["token"] == Null {
        warn!("'mail.token' not found, setting to  '!!!!!'  by default.[Mail will be disabled! Please enable after change!]");
        config["mail"]["token"] = Value::from("!!!!!");
        config["main"]["enabled"] = Value::from(false);
        edited = true;
    }
    if config["mail"]["username"] == Null {
        warn!("'mail.username' not found, setting to  '111@126.com'  by default.[Mail will be disabled! Please enable after change!]");
        config["mail"]["username"] = Value::from("111@126.com");
        config["main"]["enabled"] = Value::from(false);
        edited = true;
    }


    if config["mongodb"]["dbUrl"] == Null {
        warn!("'mongodb.dbUrl' not found, setting to  'mongodb://127.0.0.1:27017'  by default.");
        config["mongodb"]["dbUrl"] = Value::from("mongodb://127.0.0.1:27017");
        edited = true;
    }
    if config["mongodb"]["dbName"] == Null {
        warn!("'mongodb.dbName' not found, setting to  'dashboard'  by default.");
        config["mongodb"]["dbName"] = Value::from("dashboard");
        edited = true;
    }


    if edited {
        save(config);
        panic!("Config have false value(s) generated, please change and restart the server. Panic now.");
    }
}

pub fn save(config: &mut Value) {
    info!("Saving config to disk...");
    let mut file = OpenOptions::new().write(true).truncate(true).open("server.config.json").expect("Can not open 'server.config.json'");
    file.write(serde_json::to_string_pretty(config).unwrap().as_bytes()).expect("Can not write 'server.config.json'");
    info!("Config saved");
}