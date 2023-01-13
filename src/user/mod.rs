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
    let pre=crate::user::xbox::pre_auth().await;
    if let Err(err) = pre {
        warn!("500/register->{}: Preauth failed\n{}", ip.to_string(),err);
        return HttpResponse::InternalServerError().body("Preauth failed\n".to_owned()+&err);
    }
    let pre=pre.unwrap();
    let login_response=crate::user::xbox::user_login(username.to_string(), password.to_string(), pre).await;
    if let Err(err) = login_response {
        warn!("500/register->{}: Xbox Live Login failed\n{}", ip.to_string(),err);
        return HttpResponse::InternalServerError().body("Xbox Live Login failed\n".to_owned()+&err);
    }
    let login_response=login_response.unwrap();
    let xbl=crate::user::xbox::xbl_authenticate(login_response, false).await;
    if let Err(err) = xbl {
        warn!("500/register->{}: XBLAuth failed\n{}", ip.to_string(),err);
        return HttpResponse::InternalServerError().body("XBLAuth failed\n".to_owned()+&err);
    }
    let xbl=xbl.unwrap();
    let xsts=crate::user::xbox::xsts_authenticate(xbl).await;
    if let Err(err) = xsts {
        warn!("500/register->{}: XSTSAuth failed\n{}", ip.to_string(),err);
        return HttpResponse::InternalServerError().body("XSTSAuth failed\n".to_owned()+&err);
    }
    let xsts=xsts.unwrap();
    let access_token=crate::user::minecraft::login_with_xbox(xsts.user_hash, xsts.token).await;
    if let Err(err) = access_token {
        warn!("500/register->{}: Get AccessToken failed\n{}", ip.to_string(),err);
        return HttpResponse::InternalServerError().body("Get AccessToken failed\n".to_owned()+&err);
    }
    let access_token=access_token.unwrap();
    let has_game=crate::user::minecraft::user_has_game(access_token.clone()).await;
    if let Err(err) = has_game {
        warn!("500/register->{}: Get Game Status failed\n{}", ip.to_string(),err);
        return HttpResponse::InternalServerError().body("Get Game Status failed\n".to_owned()+&err);
    }
    let has_game = has_game.unwrap();
    if !has_game {
        warn!("500/register->{}: Not have game", ip.to_string());
        return HttpResponse::InternalServerError().body("Not have game");
    }
    let profile=crate::user::minecraft::get_user_profile(access_token).await;
    if let Err(err) = profile {
        warn!("500/register->{}: Could not get profile\n{}", ip.to_string(),err);
        return HttpResponse::InternalServerError().body("Could not get profile\n".to_owned()+&err);
    }
    let profile=profile.unwrap();
    match register::register(profile.uuid).await {
        Err(e) => {
            warn!("500/register->{}: {}", ip.to_string(), e.to_string());
            return HttpResponse::InternalServerError().body(e);
        }
        Ok(_)=>{
            info!("200/register->{}", ip.to_string());
            return HttpResponse::Ok().into();
        }
    }
}

#[get("/bind_qq")]
pub async fn bind_qq(req: HttpRequest, req_body: String) -> impl Responder {
    let ip = req.peer_addr().unwrap().ip();
    let content = match serde_json::from_str::<Value>(&req_body) {
        Err(e) => {
            warn!("500/bind_qq->{}: {}", ip.to_string(), e.to_string());
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
    let profile=crate::user::minecraft::get_user_profile(token.to_string()).await;
    if let Err(err) = profile {
        warn!("500/bind_qq->{}: Could not get profile\n{}", ip.to_string(),err);
        return HttpResponse::InternalServerError().body("Could not get profile\n".to_owned()+&err);
    }
    let profile=profile.unwrap();
    match bind::bind_qq(profile.uuid, qq).await {
        Ok(_) => { return HttpResponse::Ok().into(); }
        Err(e) => {
            if e != "Already Bound" {
                warn!("500/bind_qq->{}: {}", ip.to_string(), e);
                return HttpResponse::InternalServerError().body(e);
            } else {
                warn!("208/bind_qq->{}: Already bound", ip.to_string());
                return HttpResponse::AlreadyReported().body(e);
            }
        }
    }
}

#[get("/get_qq")]
pub async fn get_qq(req: HttpRequest, req_body: String) -> impl Responder {
    let ip = req.peer_addr().unwrap().ip();
    let content = match serde_json::from_str::<Value>(&req_body) {
        Err(e) => {
            warn!("500/get_qq->{}: {}", ip.to_string(), e.to_string());
            return HttpResponse::InternalServerError().body(e.to_string());
        }
        Ok(v) => v,
    };
    let uuid = match content["uuid"].as_str() {
        Some(v) => v,
        None => {
            warn!("500/get_qq->{}: UUID missing", ip.to_string());
            return HttpResponse::InternalServerError().body("UUID missing");
        }
    };
    /*let profile=crate::user::minecraft::get_user_profile(uuid.to_string()).await;
    if let Err(err) = profile {
        warn!("500/get_qq->{}: Could not get profile\n{}", ip.to_string(),err);
        return HttpResponse::InternalServerError().body("Could not get profile\n".to_owned()+&err);
    }
    let profile=profile.unwrap();*/
    match UserInfo::find_uuid(/*profile.*/uuid.to_string()).await {
        Ok(p) => { return HttpResponse::Ok().body(p.bind_qq.to_string()).into(); }
        Err(e) => {
                warn!("500/get_qq->{}: {}", ip.to_string(), e);
                return HttpResponse::InternalServerError().body(e);
        }
    }
}
