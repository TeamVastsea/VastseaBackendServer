use actix_web::{get, HttpRequest, HttpResponse, Responder};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_log::debug;
use crate::user::login::login_password;

pub mod register;
pub mod login;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    username: String,
    mail_box: String,
    group: Vec<String>,
    enabled: bool,
    bind_user: String,
    bind_qq: i64,
    otp: bool,
    password: String,
    salt: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenInfo {
    username: String,
    salt: String,
    ip: String,
}

#[get("/login")]
pub async fn login_request(req: HttpRequest, req_body: String) -> impl Responder{
    let content = match serde_json::from_str::<Value>(&req_body) {
        Err(e) => {return HttpResponse::InternalServerError().body(e.to_string());},
        Ok(v) => v,
    };
    let username = match content["username"].as_str() {
        Some(v) => v,
        None => {return HttpResponse::InternalServerError().body("Username missing");}
    };
    let password = match content["password"].as_str() {
        Some(v) => v,
        None => {return HttpResponse::InternalServerError().body("Password missing");}
    };
    let ip = req.peer_addr().unwrap().ip();
    match login_password(username.to_string(), password.to_string(), ip.to_string()).await {
        Ok(v) => HttpResponse::Ok().body(doc!{"available": true, "content": v}.to_string()),
        Err(e) => HttpResponse::Unauthorized().body(doc!{"available": false, "content": e}.to_string()),
    }
}