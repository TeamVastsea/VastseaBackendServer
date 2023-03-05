mod api_key;

use std::{io};
use chrono::{DateTime, Utc};


use serde::{Deserialize, Serialize};
use crate::command::api_key::get_api_key;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ApiKey {
    _id: String,
    description: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    nbf: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    nat: DateTime<Utc>,

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
    let mut buffer = "".to_string();
    let stdin = io::stdin();

    //init routes
    // let mut routes = Router::new();
    // routes.add_route("key", Arc::new(|params| get_api_key(params)));

    loop {
        match stdin.read_line(&mut buffer) {
            Ok(_) => {}
            Err(e) => {println!("Cannot get input: {}" , e);}
        };

        let input = buffer[..buffer.len() - 1].to_string();
        router(input).await;
        buffer = "".to_string();
    }
}

async fn router(input: String) {
    let params:Vec<&str> = input.split(" ").collect();
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
        &_ => { println!("Cannot match command'{}'", name); }
    }
}