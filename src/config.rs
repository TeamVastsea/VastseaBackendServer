use std::fs::OpenOptions;
use std::io::Write;
use rand::distributions::{Alphanumeric, Distribution};
use serde_json::Value;
use serde_json::Value::Null;
use simple_log::{info, warn};

pub fn init(config: &mut Value) {
    info!("Checking configs...");
    let mut edited: bool = false;
    let mut rng = rand::thread_rng();


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
        warn!("'defaultUserGroup' not found, setting to  'user'  by default.");
        config["defaultUserGroup"] = Value::from("user");
        edited = true;
    }
    if config["pluginPassword"] == Null {
        let pass: String = Alphanumeric.sample_iter(&mut rng).take(16).map(char::from).collect::<String>().to_lowercase();
        warn!("'baseUrl' not found, setting to  '{}'  by default.", pass);
        config["baseUrl"] = Value::from(pass);
        edited = true;
    }
    if config["tokenKey"] == Null {
        let pass: String = Alphanumeric.sample_iter(&mut rng).take(16).map(char::from).collect::<String>().to_lowercase();
        warn!("'tokenKey' not found, setting to  '{}'  by default.", pass);
        config["tokenKey"] = Value::from(pass);
        edited = true;
    }


    if config["connection"]["serverPort"] == Null {
        warn!("'connection.serverPort' not found, setting to  7890  by default.");
        config["connection"]["serverPort"] = Value::from(7890);
        edited = true;
    }
    if config["connection"]["sslCert"] == Null {
        warn!("'connection.sslCert' not found, setting to  '.\\keys\\mydomain.crt'  by default.");
        config["connection"]["sslCert"] = Value::from(".\\keys\\mydomain.crt");
        edited = true;
    }
    if config["connection"]["sslKey"] == Null {
        warn!("'connection.sslKey' not found, setting to  '.\\keys\\private.key'  by default.");
        config["connection"]["sslKey"] = Value::from(".\\keys\\private.key");
        edited = true;
    }


    if config["mail"]["secureConnection"] == Null {
        warn!("'mail.secureConnection' not found, setting to  false  by default.");
        config["mail"]["secureConnection"] = Value::from(false);
        edited = true;
    }
    if config["mail"]["smtpHost"] == Null {
        warn!("'mail.smtpHost' not found, setting to  'smtp.126.com'  by default.");
        config["mail"]["smtpHost"] = Value::from("smtp.126.com");
        edited = true;
    }
    if config["mail"]["smtpPort"] == Null {
        warn!("'mail.smtpPort' not found, setting to  25  by default.");
        config["mail"]["smtpPort"] = Value::from(25);
        edited = true;
    }
    if config["mail"]["token"] == Null {
        warn!("'mail.token' not found, setting to  '!!!!!'  by default.");
        config["mail"]["tokoen"] = Value::from("!!!!!");
        edited = true;
    }
    if config["mail"]["username"] == Null {
        warn!("'mail.username' not found, setting to  '111@126.com'  by default.");
        config["mail"]["username"] = Value::from("111@126.com");
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
    }
}

pub fn save(config: &mut Value) {
    info!("Saving config to disk...");
    let mut file = OpenOptions::new().write(true).truncate(true).open("server.config.json").expect("Can not open 'server.config.json'");
    file.write(serde_json::to_string_pretty(config).unwrap().as_bytes()).expect("Can not write 'server.config.json'");
    info!("Config saved");
}