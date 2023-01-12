use actix_web::{get, HttpRequest, HttpResponse, Responder};
use mongodb::bson::{doc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_log::{info, warn};

pub mod register;
pub mod bind;
pub mod microsoft;
pub mod xbox;
pub mod minecraft;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInfo {
    pub bind_qq: i64,
    pub uuid: String,
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
