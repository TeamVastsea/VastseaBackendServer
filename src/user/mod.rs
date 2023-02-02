use std::borrow::Cow;
use actix_web::{get, post, HttpRequest, HttpResponse, Responder};
use mongodb::bson::{doc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_log::{info, warn};
use url_encoded_data::UrlEncodedData;
use crate::user::microsoft::{LoginResponse, request_access_token};
use crate::user::minecraft::{get_user_profile, login_with_xbox, user_has_game};
use crate::user::xbox::{xbl_authenticate, xsts_authenticate};

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

#[post("/password")]
async fn password_login(req: HttpRequest, req_body: String) -> impl Responder {
    let ip = req.peer_addr().unwrap().ip();
    let content = match serde_json::from_str::<Value>(req_body.as_str()) {
        Err(e) => {
            warn!("500/register->{}: {}", ip.to_string(), e.to_string());
            return HttpResponse::InternalServerError().body(e.to_string());
        }
        Ok(v) => v,
    };

    let username = match content.get("username") {
        Some(v) => v.as_str().unwrap().to_string(),
        None => {
            warn!("500/register->{}: Username missing", ip.to_string());
            return HttpResponse::InternalServerError().body("Username missing");
        }
    };
    let password = match content.get("password") {
        Some(v) => v.as_str().unwrap().to_string(),
        None => {
            warn!("500/register->{}: Password missing", ip.to_string());
            return HttpResponse::InternalServerError().body("Password missing");
        }
    };
    let pre = match xbox::pre_auth().await {
        Err(e) => {
            warn!("500/register->{}: Pre-auth failed\n{}", ip.to_string(),e);
            return HttpResponse::InternalServerError().body("Pre-auth failed\n".to_owned() + &e);
        }
        Ok(a) => a,
    };
    let login_response = match xbox::user_login(username.to_string(), password.to_string(), pre).await {
        Err(e) => {
            warn!("500/register->{}: Xbox Live Login failed\n{}", ip.to_string(),e);
            return HttpResponse::InternalServerError().body("Xbox Live Login failed\n".to_owned() + &e);
        }
        Ok(a) => a,
    };
    let xbl = match xbl_authenticate(login_response, false).await {
        Err(e) => {
            warn!("500/register->{}: XBLAuth failed\n{}", ip.to_string(),e);
            return HttpResponse::InternalServerError().body("XBLAuth failed\n".to_owned() + &e);
        }
        Ok(a) => a
    };
    let xsts = match xsts_authenticate(xbl).await {
        Err(e) => {
            warn!("500/register->{}: XSTSAuth failed\n{}", ip.to_string(),e);
            return HttpResponse::InternalServerError().body("XSTSAuth failed\n".to_owned() + &e);
        }
        Ok(a) => a,
    };
    let access_token = match login_with_xbox(xsts.user_hash, xsts.token).await {
        Err(err) => {
            warn!("500/register->{}: Get AccessToken failed\n{}", ip.to_string(),err);
            return HttpResponse::InternalServerError().body("Get AccessToken failed\n".to_owned() + &err);
        }
        Ok(a) => a,
    };
    let has_game = match user_has_game(access_token.clone()).await {
        Err(err) => {
            warn!("500/register->{}: Get Game Status failed\n{}", ip.to_string(),err);
            return HttpResponse::InternalServerError().body("Get Game Status failed\n".to_owned() + &err);
        }
        Ok(a) => a,
    };
    if !has_game {
        warn!("500/register->{}: Not have game", ip.to_string());
        return HttpResponse::Unauthorized().body("Not have game");
    }
    let profile = match minecraft::get_user_profile(access_token).await {
        Err(err) => {
            warn!("500/register->{}: Could not get profile\n{}", ip.to_string(),err);
            return HttpResponse::InternalServerError().body("Could not get profile\n".to_owned() + &err);
        }
        Ok(a) => a,
    };
    return match register::register(profile.uuid).await {
        Err(e) => {
            warn!("500/register->{}: {}", ip.to_string(), e.to_string());
            HttpResponse::InternalServerError().body(e)
        }
        Ok(_) => {
            info!("200/register->{}", ip.to_string());
            HttpResponse::Ok().into()
        }
    };
}

#[get("/login_code")]
pub async fn code_login(req: HttpRequest, _req_body: String) -> impl Responder {
    let uri = req.uri().to_string();
    let uri_encoded = UrlEncodedData::from(uri.as_str());
    let ip = req.peer_addr().unwrap().ip();

    if !uri_encoded.keys().contains(&"code") {
        warn!("400/code->{}: Missing 'code'", ip.to_string());
        return HttpResponse::BadRequest().body("Missing 'token'");
    }

    let code = uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("code")).unwrap().to_string();
    let need_token = match uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("token")) {
        None => false,
        Some(a) => {
            if a.to_string() == "true" {
                true
            } else {
                false
            }
        }
    };
    let access_token = match request_access_token(code.to_string()).await {
        Ok(a) => a,
        Err(e) => {
            warn!("500/code->{}: Cannot get access token ({})", ip.to_string(),e);
            return HttpResponse::InternalServerError().body(format!("Cannot get access token ({})", e));
        }
    };
    let xbl_response = match xbl_authenticate(access_token, true).await {
        Ok(a) => a,
        Err(e) => {
            warn!("500/code->{}: Cannot get xbl ({})", ip.to_string(),e);
            return HttpResponse::InternalServerError().body(format!("Cannot get xbl ({})", e));
        }
    };
    let xsts_response = match xsts_authenticate(xbl_response).await {
        Ok(a) => a,
        Err(e) => {
            warn!("500/code->{}: Cannot get xsts ({})", ip.to_string(),e);
            return HttpResponse::InternalServerError().body(format!("Cannot get xsts ({})", e));
        }
    };
    let xbox_token = match login_with_xbox(xsts_response.user_hash, xsts_response.token).await {
        Ok(a) => a,
        Err(e) => {
            warn!("500/code->{}: Cannot login xbox ({})", ip.to_string(),e);
            return HttpResponse::InternalServerError().body(format!("Cannot login xbox ({})", e));
        }
    };
    let has_game = match user_has_game(xbox_token.clone()).await {
        Ok(a) => a,
        Err(e) => {
            warn!("500/code->{}: Cannot examine whether has game ({})", ip.to_string(),e);
            return HttpResponse::InternalServerError().body(format!("Cannot examine whether has game ({})", e));
        }
    };
    if !has_game {
        warn!("401/code->{}: User does not own Minecraft", ip.to_string());
        return HttpResponse::Unauthorized().body("User does not own Minecraft");
    }
    return match get_user_profile(xbox_token.clone()).await {
        Ok(mut a) => {
            if need_token {
                a.token = Some("Unimplemented.".to_string());
            }

            info!("200/code->{}: {}", ip.to_string(), serde_json::to_string(&a).unwrap());
            HttpResponse::Ok().body(serde_json::to_string(&a).unwrap())
        }
        Err(e) => {
            warn!("500/code->{}: Cannot get profile ({})", ip.to_string(),e);
            HttpResponse::InternalServerError().body(format!("Cannot get profile ({})", e))
        }
    };
}

#[get("/bind_qq")]
pub async fn bind_qq(req: HttpRequest, _req_body: String) -> impl Responder {
    let ip = req.peer_addr().unwrap().ip();
    let uri = req.uri().to_string();
    let mut arg = uri.split("?");
    if arg.clone().count() <= 1 {
        warn!("500/bind_qq->{}: {}", ip.to_string(), "missing args");
        return HttpResponse::InternalServerError().body("missing args");
    }
    let arg = urlencoding::decode(arg.nth(1).unwrap()).unwrap();
    let content = match serde_json::from_str::<Value>(&arg) {
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
    let profile = minecraft::get_user_profile(token.to_string()).await;
    if let Err(err) = profile {
        warn!("500/bind_qq->{}: Could not get profile\n{}", ip.to_string(),err);
        return HttpResponse::InternalServerError().body("Could not get profile\n".to_owned() + &err);
    }
    let profile = profile.unwrap();
    return match bind::bind_qq(profile.uuid, qq).await {
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
    };
}

#[get("/get_qq")]
pub async fn get_qq(req: HttpRequest, _req_body: String) -> impl Responder {
    let ip = req.peer_addr().unwrap().ip();
    let uri = req.uri().to_string();
    let mut arg = uri.split("?");
    if arg.clone().count() <= 1 {
        warn!("500/get_qq->{}: {}", ip.to_string(), "missing args");
        return HttpResponse::InternalServerError().body("missing args");
    }
    let arg = urlencoding::decode(arg.nth(1).unwrap()).unwrap();
    let content = match serde_json::from_str::<Value>(&arg) {
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
    return match UserInfo::find_uuid(/*profile.*/uuid.to_string()).await {
        Ok(p) => { HttpResponse::Ok().body(p.bind_qq.to_string()).into() }
        Err(e) => {
            warn!("500/get_qq->{}: {}", ip.to_string(), e);
            HttpResponse::InternalServerError().body(e)
        }
    };
}
