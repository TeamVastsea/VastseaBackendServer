use std::borrow::Cow;
use actix_web::{get, HttpRequest, HttpResponse, Responder, patch};
use actix_web::web::Json;
use mongodb::bson::{doc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_log::{debug, info, warn};
use url_encoded_data::UrlEncodedData;

use crate::user::minecraft::{get_user_profile};


pub mod bind;
pub mod microsoft;
pub mod xbox;
pub mod minecraft;
mod info;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInfo {
    pub _id: String,
    pub display_name: String,
    pub enabled: bool,
    pub group: Vec<String>,
    pub bind_qq: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserMCProfile {
    pub uuid: String,
    pub user_name: String,
}

// #[post("/password")]
// async fn password_login(req: HttpRequest, _req_body: String) -> impl Responder {
//     let uri = req.uri().to_string();
//     let uri_encoded = UrlEncodedData::from(uri.as_str());
//     let ip = req.peer_addr().unwrap().ip();
//
//     if !uri_encoded.keys().contains(&"username") {
//         warn!("400/code->{}: Missing 'username'", ip.to_string());
//         return HttpResponse::BadRequest().body("Missing 'username'");
//     }
//     if !uri_encoded.keys().contains(&"password") {
//         warn!("400/code->{}: Missing 'password'", ip.to_string());
//         return HttpResponse::BadRequest().body("Missing 'password'");
//     }
//
//     let username = uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("username")).unwrap().to_string();
//     let password = uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("password")).unwrap().to_string();
//
//     let pre = match xbox::pre_auth().await {
//         Err(e) => {
//             warn!("500/register->{}: Pre-auth failed\n{}", ip.to_string(),e);
//             return HttpResponse::InternalServerError().body("Pre-auth failed\n".to_owned() + &e);
//         }
//         Ok(a) => a,
//     };
//     let login_response = match xbox::user_login(username.to_string(), password.to_string(), pre).await {
//         Err(e) => {
//             warn!("500/register->{}: Xbox Live Login failed\n{}", ip.to_string(),e);
//             return HttpResponse::InternalServerError().body("Xbox Live Login failed\n".to_owned() + &e);
//         }
//         Ok(a) => a,
//     };
//     let xbl = match xbl_authenticate(login_response, false).await {
//         Err(e) => {
//             warn!("500/register->{}: XBLAuth failed\n{}", ip.to_string(),e);
//             return HttpResponse::InternalServerError().body("XBLAuth failed\n".to_owned() + &e);
//         }
//         Ok(a) => a
//     };
//     let xsts = match xsts_authenticate(xbl).await {
//         Err(e) => {
//             warn!("500/register->{}: XSTSAuth failed\n{}", ip.to_string(),e);
//             return HttpResponse::InternalServerError().body("XSTSAuth failed\n".to_owned() + &e);
//         }
//         Ok(a) => a,
//     };
//     let access_token = match login_with_xbox(xsts.user_hash, xsts.token).await {
//         Err(err) => {
//             warn!("500/register->{}: Get AccessToken failed\n{}", ip.to_string(),err);
//             return HttpResponse::InternalServerError().body("Get AccessToken failed\n".to_owned() + &err);
//         }
//         Ok(a) => a,
//     };
//     let has_game = match user_has_game(access_token.clone()).await {
//         Err(err) => {
//             warn!("500/register->{}: Get Game Status failed\n{}", ip.to_string(),err);
//             return HttpResponse::InternalServerError().body("Get Game Status failed\n".to_owned() + &err);
//         }
//         Ok(a) => a,
//     };
//     if !has_game {
//         warn!("500/register->{}: Not have game", ip.to_string());
//         return HttpResponse::Unauthorized().body("Not have game");
//     }
//     let profile = match get_user_profile(access_token).await {
//         Err(err) => {
//             warn!("500/register->{}: Could not get profile\n{}", ip.to_string(),err);
//             return HttpResponse::InternalServerError().body("Could not get profile\n".to_owned() + &err);
//         }
//         Ok(a) => a,
//     };
//     return match register::register(profile.uuid, profile.user_name).await {
//         Err(e) => {
//             warn!("500/register->{}: {}", ip.to_string(), e.to_string());
//             HttpResponse::InternalServerError().body(e)
//         }
//         Ok(_) => {
//             info!("200/register->{}", ip.to_string());
//             HttpResponse::Ok().into()
//         }
//     };
// }

#[get("/user")]
pub async fn user_get(req: HttpRequest, _req_body: String) -> impl Responder {
    let uri = req.uri().to_string();
    let uri_encoded = UrlEncodedData::from(uri.as_str());
    let ip = req.peer_addr().unwrap().ip();
    let mut method = "";
    let mut user_info: UserInfo = UserInfo {
        _id: "".to_string(),
        display_name: "".to_string(),
        enabled: false,
        group: vec![],
        bind_qq: None,
    };

    if uri_encoded.keys().contains(&"code") {
        method = "code";
        let code = uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("code")).unwrap().to_string();
        let mc_profile = match UserMCProfile::from_code(code).await {
            Ok(a) => a,
            Err(err) => {
                warn!("500/user->{}: {}", ip.to_string(), &err);
                return HttpResponse::InternalServerError().body(err);
            }
        };
        user_info = match UserInfo::from_mc_profile(mc_profile).await {
            Ok(a) => a,
            Err(err) => {
                warn!("500/user->{}: {}", ip.to_string(), &err);
                return HttpResponse::InternalServerError().body(err);
            }
        }
    }
    if uri_encoded.keys().contains(&"htoken") {
        method = "token";
        let token = uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("htoken")).unwrap().to_string();
        user_info = match UserInfo::from_token(token).await {
            Ok(a) => a,
            Err(err) => {
                warn!("500/user->{}: {}", ip.to_string(), &err);
                return HttpResponse::InternalServerError().body(err);
            }
        };
    }
    // if uri_encoded.keys().contains(&"username") && uri_encoded.keys().contains(&"password") {
    //     method = "username";
    // }

    if method == "" {
        warn!("400/user->{}: Missing args", ip.to_string());
        return HttpResponse::BadRequest().body("Missing args");
    }

    let mut result = serde_json::to_value(user_info.clone()).unwrap();

    if let Some(a) = uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("token")) {
        if a.to_string() == "true" {
            info!("200/user->{}", ip.to_string());
            result["token"] = Value::from(user_info.to_token().await);
        }
    };

    return HttpResponse::Ok().body(result.to_string());
}

#[patch("/bind_qq")]
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
    let profile = match get_user_profile(token.to_string()).await {
        Err(err) => {
            warn!("500/bind_qq->{}: Could not get profile\n{}", ip.to_string(),err);
            return HttpResponse::InternalServerError().body("Could not get profile\n".to_owned() + &err);
        }
        Ok(a) => a,
    };
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