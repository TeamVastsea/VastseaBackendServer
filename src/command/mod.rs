mod api_key;


use std::io;
use std::process::exit;
use chrono::{DateTime, Utc};


use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;
use crate::command::api_key::get_api_key;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiKey {
    _id: String,
    usage: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub nbf: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub nat: DateTime<Utc>,

}

// type Handler = Arc<dyn Fn(Vec<String>)>;
//
// struct Router {
//     routes: HashMap<String, Handler>,
// }
//
// impl Router {
//     fn new() -> Self {
//         Router { routes: HashMap::new() }
//     }
//
//     fn add_route(&mut self, path: &str, handler: Handler) {
//         self.routes.insert(path.to_string(), handler);
//     }
//
//     fn route(&self, path: &str, params: Vec<String>) -> Result<(), ()> {
//         if let Some(handler) = self.routes.get(path) {
//             handler(params);
//             Ok(())
//         } else {
//             Err(())
//         }
//     }
// }

// thread_local! {static ROUTES: Router = Router::new();}

pub async fn listener() {
    println!("000");
    let mut buffer = "".to_string();
    let mut stdin = io::stdin();

    //init routes
    // let mut routes = Router::new();
    // routes.add_route("key", Arc::new(|params| get_api_key(params)));

    loop {
        print!("111");
        match stdin.read_line(&mut buffer) {
            Ok(_) => {}
            Err(e) => { println!("Cannot get input: {}", e); }
        };
        println!("222");

        let input = buffer.trim_end().to_string();
        router(input).await;
        buffer.clear();
    }
}

async fn router(input: String) {
    let params: Vec<&str> = input.split(" ").collect();
    if params.is_empty() || params.len() < 1 {
        println!("Invalid input");
    }
    let name = params[0];
    let mut input: Vec<String> = vec![];
    for i in 1..params.len() {
        input.push(params[i].to_string());
    }

    match name {
        "key" => { get_api_key(input).await; }
        "stop" => { exit(0); }
        &_ => { println!("Cannot match command'{}'", name); }
    }
}