use actix_web::{get, HttpRequest, HttpResponse, Responder};
use mongodb::bson::{DateTime, doc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_log::{info, warn};
use crate::user::email::verify_email;

use crate::user::login::login_password;

pub mod register;
pub mod login;
pub mod email;
pub mod change;
pub mod bind;

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
    ip: Option<String>,
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

#[derive(Serialize, Deserialize, Clone)]
pub struct BindTokenInfo {
    username: String,
    insert_time: DateTime,
    token: Option<String>,
}

#[get("/login")]
pub async fn login_request(req: HttpRequest, req_body: String) -> impl Responder {
    let ip = req.peer_addr().unwrap().ip();
    let content = match serde_json::from_str::<Value>(&req_body) {
        Err(e) => {
            warn!("500/login->{}: {}", ip.to_string(), e.to_string());
            return HttpResponse::InternalServerError().body(e.to_string());
        }
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
        }
        Err(e) => {
            warn!("401/login->{}: {}", ip.to_string(), e);
            HttpResponse::Unauthorized().body(doc! {"available": false, "content": e}.to_string())
        }
    }
}

#[get("/register")]
pub async fn register_request(req: HttpRequest, req_body: String) -> impl Responder {
    let ip = req.peer_addr().unwrap().ip();
    let content = match serde_json::from_str::<Value>(&req_body) {
        Err(e) => {
            warn!("500/register->{}: {}", ip.to_string(), e.to_string());
            return HttpResponse::InternalServerError().body(e.to_string());
        }
        Ok(v) => v,
    };
    let username = match content["username"].as_str() {
        Some(v) => v,
        None => {
            warn!("500/register->{}: Username missing", ip.to_string());
            return HttpResponse::InternalServerError().body("Username missing");
        }
    };
    let password = match content["password"].as_str() {
        Some(v) => v,
        None => {
            warn!("500/register->{}: Password missing", ip.to_string());
            return HttpResponse::InternalServerError().body("Password missing");
        }
    };
    let mailbox = match content["mailbox"].as_str() {
        Some(v) => v,
        None => {
            warn!("500/register->{}: Mailbox missing", ip.to_string());
            return HttpResponse::InternalServerError().body("Mailbox missing");
        }
    };
    match register::register(username.to_string(), password.to_string(), mailbox.to_string(), &ip.to_string()).await {
        Err(e) => {
            warn!("500/register->{}: {}", ip.to_string(), e.to_string());
            return HttpResponse::InternalServerError().body(e);
        }
        Ok(user) => { user.send_verify_email().await; }
    }
    info!("200/register->{}", ip.to_string());
    HttpResponse::Ok().into()
}

#[get("/verify_email")]
pub async fn verify_email_request(req: HttpRequest, req_body: String) -> impl Responder {
    let ip = req.peer_addr().unwrap().ip();
    let content = match serde_json::from_str::<Value>(&req_body) {
        Err(e) => {
            warn!("500/verify_email->{}: {}", ip.to_string(), e.to_string());
            return HttpResponse::InternalServerError().body(e.to_string());
        }
        Ok(v) => v,
    };
    let token = match content["token"].as_str() {
        Some(v) => v,
        None => {
            warn!("500/verify_email->{}: {}", ip.to_string(), "Token missing");
            return HttpResponse::InternalServerError().body("Token missing");
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
            info!("200/verify_email->{}", ip.to_string());
            HttpResponse::Ok().into()
        }
        Err(e) => {
            warn!("500/verify_email->{}: {}", ip.to_string(), e);
            HttpResponse::InternalServerError().body(e)
        }
    }
}

#[get("/bind_token")]
pub async fn bind_token(req: HttpRequest, req_body: String) -> impl Responder {
    let ip = req.peer_addr().unwrap().ip();
    let content = match serde_json::from_str::<Value>(&req_body) {
        Err(e) => {
            warn!("500/bing_token->{}: {}", ip.to_string(), e.to_string());
            return HttpResponse::InternalServerError().body(e.to_string());
        }
        Ok(v) => v,
    };
    let token = match content["token"].as_str() {
        Some(v) => v,
        None => {
            warn!("500/bind_token->{}: Token missing", ip.to_string());
            return HttpResponse::InternalServerError().body("Token missing");
        }
    };
    let user = match login::login_token(token, ip.to_string().as_str()).await {
        Ok(a) => a,
        Err(_) => {
            warn!("500/bind_token->{}", ip.to_string());
            return HttpResponse::InternalServerError().into();
        }
    };
    let mut token_info = BindTokenInfo {
        username: user.username,
        insert_time: DateTime::now(),
        token: None,
    };
    let token = token_info.register_token().await;
    info!("200/bind_token->{}: token", ip.to_string());
    HttpResponse::Ok().body(token)
}

#[get("/bind_qq")]
pub async fn bind_qq(req: HttpRequest, req_body: String) -> impl Responder {
    let ip = req.peer_addr().unwrap().ip();
    let content = match serde_json::from_str::<Value>(&req_body) {
        Err(e) => {
            warn!("500/bing_qq->{}: {}", ip.to_string(), e.to_string());
            return HttpResponse::InternalServerError().body(e.to_string());
        }
        Ok(v) => v,
    };
    let token = match content["token"].as_str() {
        Some(v) => v,
        None => {
            warn!("500/bind_qq->{}: Token missing", ip.to_string());
            return HttpResponse::InternalServerError().body("Token missing");
        }
    };
    let qq = match content["qq"].as_i64() {
        Some(v) => v,
        None => {
            warn!("500/bind_qq->{}: QQ missing", ip.to_string());
            return HttpResponse::InternalServerError().body("QQ missing");
        }
    };
    match bind::bind_qq(token.to_string(), qq).await {
        Ok(_) => { HttpResponse::Ok().into() }
        Err(e) => {
            if e != "Already Bound" {
                warn!("500/bind_qq->{}: {}", ip.to_string(), e);
                HttpResponse::InternalServerError().body(e)
            } else {
                warn!("208/bind_qq->{}: Already bound", ip.to_string());
                HttpResponse::AlreadyReported().body(e)
            }
        }
    }
}