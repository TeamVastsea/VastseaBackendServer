use actix_web::{get, HttpRequest, HttpResponse, Responder};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_log::{info, warn};
use crate::user::email::verify_email;

use crate::user::login::login_password;

pub mod register;
pub mod login;
pub mod email;

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub ip: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TokenInfo {
    username: String,
    salt: String,
    ip: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EmailTokenInfo {
    username: String,
    mail_box: String,
    token: String,
    time: String,
    ip: String,
}

#[get("/login")]
pub async fn login_request(req: HttpRequest, req_body: String) -> impl Responder{
    let ip = req.peer_addr().unwrap().ip();
    let content = match serde_json::from_str::<Value>(&req_body) {
        Err(e) => {
            warn!("500/login->{}: {}", ip.to_string(), e.to_string());
            return HttpResponse::InternalServerError().body(e.to_string());
        },
        Ok(v) => v,
    };
    let username = match content["username"].as_str() {
        Some(v) => v,
        None => {
            warn!("500/login->{}: {}", ip.to_string(), "Username missing");
            return HttpResponse::InternalServerError().body("Username missing");
        }
    };
    let password = match content["password"].as_str() {
        Some(v) => v,
        None => {
            warn!("500/login->{}: {}", ip.to_string(), "Password missing");
            return HttpResponse::InternalServerError().body("Password missing");
        }
    };
    match login_password(username.to_string(), password.to_string(), ip.to_string()).await {
        Ok((_, v)) => {
            info!("200/login->{}: {}", ip.to_string(), "token".to_string());
            HttpResponse::Ok().body(doc! {"available": true, "content": v}.to_string())
        },
        Err(e) => {
            warn!("401/login->{}: {}", ip.to_string(), e);
            HttpResponse::Unauthorized().body(doc! {"available": false, "content": e}.to_string())
        },
    }
}

#[get("/register")]
pub async fn register_request(req: HttpRequest, req_body: String) -> impl Responder{
    let ip = req.peer_addr().unwrap().ip();
    let content = match serde_json::from_str::<Value>(&req_body) {
        Err(e) => {
            warn!("500/login->{}: {}", ip.to_string(), e.to_string());
            return HttpResponse::InternalServerError().body(e.to_string());
        },
        Ok(v) => v,
    };
    let username = match content["username"].as_str() {
        Some(v) => v,
        None => {
            warn!("500/login->{}: {}", ip.to_string(), "Username missing");
            return HttpResponse::InternalServerError().body("Username missing");
        }
    };
    let password = match content["password"].as_str() {
        Some(v) => v,
        None => {
            warn!("500/login->{}: {}", ip.to_string(), "Password missing");
            return HttpResponse::InternalServerError().body("Password missing");
        }
    };
    let mailbox = match content["mailbox"].as_str() {
        Some(v) => v,
        None => {
            warn!("500/login->{}: {}", ip.to_string(), "Mailbox missing");
            return HttpResponse::InternalServerError().body("Mailbox missing");
        }
    };
    match register::register(username.to_string(), password.to_string(), mailbox.to_string(), &ip.to_string()).await {
        Err(e) => {
            warn!("500/login->{}: {}", ip.to_string(), e.to_string());
            return HttpResponse::InternalServerError().body(e);
        },
        Ok(user) => {user.send_verify_email().await;}
    }
    HttpResponse::Ok().body("0")
}

#[get("/verify_email")]
pub async fn verify_email_request(req: HttpRequest, req_body: String) -> impl Responder{
    let ip = req.peer_addr().unwrap().ip();
    let content = match serde_json::from_str::<Value>(&req_body) {
        Err(e) => {
            warn!("500/verify_email->{}: {}", ip.to_string(), e.to_string());
            return HttpResponse::InternalServerError().body(e.to_string());
        },
        Ok(v) => v,
    };
    let token = match content["token"].as_str() {
        Some(v) => v,
        None => {
            warn!("500/verify_email->{}: {}", ip.to_string(), "Mailbox missing");
            return HttpResponse::InternalServerError().body("Mailbox missing");
        }
    };
    let password = match content["password"].as_str() {
        Some(v) => v,
        None => {
            warn!("500/verify_email->{}: {}", ip.to_string(), "Password missing");
            return HttpResponse::InternalServerError().body("Password missing");
        }
    };
    match verify_email(token.to_string(), password.to_string()).await {
        Ok(_) => {
            info!("200/verify_email->{}: {}", ip.to_string(), "0".to_string());
            HttpResponse::Ok().body("0")
        },
        Err(e) => {
            warn!("500/verify_email->{}: {}", ip.to_string(), e);
            HttpResponse::InternalServerError().body(e)
        }
    }
}
