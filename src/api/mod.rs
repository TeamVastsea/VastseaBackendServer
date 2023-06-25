mod bind;
mod key;
mod ban;
mod luck;

use std::borrow::Cow;
use actix_web::{HttpRequest, HttpResponse, patch, put, get, Responder};
use chrono::{DateTime, Utc};
use simple_log::{info, warn};
use url_encoded_data::UrlEncodedData;
use serde::{Deserialize, Serialize};
use crate::api::ban::{ban_user, ban_user_qq};
use crate::api::bind::bind_qq;
use crate::api::key::examine_key;
use crate::api::luck::calc_luck;
use crate::user::{UserInfo};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiKey {
    _id: String,
    usage: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub nbf: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub nat: DateTime<Utc>
}

#[patch("/users")]
pub async fn user_patch(req: HttpRequest, _req_body: String) -> impl Responder {
    let uri = req.uri().to_string();
    let uri_encoded = UrlEncodedData::from(uri.as_str());
    let ip = req.peer_addr().unwrap().ip().to_string();

    if !uri_encoded.exists("uuid") || !uri_encoded.exists("qq") || !uri_encoded.exists("key") {
        warn!("400/users(patch)->{}: Missing argument(s).", ip.to_string());
        return HttpResponse::BadRequest().body("Missing argument(s).");
    }

    let uuid = uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("name")).unwrap().to_string();
    let qq = uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("qq")).unwrap().to_string();
    let qq = match qq.parse::<i64>() {
        Ok(a) => {a}
        Err(err) => {
            warn!("400/users(patch)->{}: Missing argument(s).", ip.to_string());
            return HttpResponse::BadRequest().body("Cannot parse qq: ".to_string() + err.to_string().as_str());
        }
    };
    let key = uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("key")).unwrap().to_string();
    if examine_key(key).await.is_err() {
        warn!("401/users(patch)->{}: Wrong key.", ip.to_string());
        return HttpResponse::Unauthorized().body("Wrong key.");
    }

    return match bind_qq(uuid, qq).await {
        Ok(_) => {
            info!("200/users(patch)->{}", ip.to_string());
            HttpResponse::Ok().body("")
        }
        Err(_) => {
            warn!("500/users(patch)->{}: User not found or already bound.", ip.to_string());
            HttpResponse::InternalServerError().body("User not found or already bound.")
        }
    };
}

#[put("/users")]
pub async fn user_put(req: HttpRequest, _req_body: String) -> impl Responder {

    let uri = req.uri().to_string();
    let uri_encoded = UrlEncodedData::from(uri.as_str());
    let ip = req.peer_addr().unwrap().ip().to_string();

    if !((uri_encoded.exists("uuid") || uri_encoded.exists("qq")) && uri_encoded.exists("key")) {
        warn!("400/users(patch)->{}: Missing argument(s).", ip.to_string());
        return HttpResponse::BadRequest().body("Missing argument(s).");
    }
    let key = uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("key")).unwrap().to_string();

    let reason = if uri_encoded.exists("reason") {
        uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("reason")).unwrap().to_string()
    } else {
        "".to_string()
    };

    if examine_key(key).await.is_err() {
        warn!("401/users(patch)->{}: Wrong key.", ip.to_string());
        return HttpResponse::Unauthorized().body("Wrong key.");
    }

    return if uri_encoded.exists("uuid") {
        let uuid = uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("uuid")).unwrap().to_string();
        match ban_user(uuid, reason).await {
            Ok(_) => {
                info!("200/users(patch)->{}", ip.to_string());
                HttpResponse::Ok().body("")
            }
            Err(_) => {
                warn!("500/users(patch)->{}: User not found or already disabled.", ip.to_string());
                HttpResponse::InternalServerError().body("User not found or already disabled.")
            }
        }
    } else {
        let qq = match uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("qq")).unwrap().to_string().parse::<i64>() {
            Ok(a) => a,
            Err(err) => {
                warn!("500/users(patch)->{}: Cannot parse qq: {}.", ip.to_string(), err.to_string());
                return HttpResponse::InternalServerError().body("cannot parse qq: ".to_string() + err.to_string().as_str() + ".");
            }
        };
        match ban_user_qq(qq, reason).await {
            Ok(_) => {
                info!("200/users(patch)->{}", ip.to_string());
                HttpResponse::Ok().body("")
            }
            Err(_) => {
                warn!("500/users(patch)->{}: User not found or already disabled.", ip.to_string());
                HttpResponse::InternalServerError().body("User not found or already disabled.")
            }
        }
    }
}

#[get("/user/qq")]
pub async fn user_qq_get(req: HttpRequest, _req_body: String) -> impl Responder {
    let uri = req.uri().to_string();
    let uri_encoded = UrlEncodedData::from(uri.as_str());
    let ip = req.peer_addr().unwrap().ip().to_string();

    if !uri_encoded.exists("uuid") || !uri_encoded.exists("key") {
        warn!("400/user/qq(get)->{}: Missing argument(s).", ip.to_string());
        return HttpResponse::BadRequest().body("Missing argument(s).");
    }

    let key = uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("key")).unwrap().to_string();

    if examine_key(key).await.is_err() {
        warn!("401/user/qq(get)->{}: Wrong key.", ip.to_string());
        return HttpResponse::Unauthorized().body("Wrong key.");
    }

    let uuid = uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("uuid")).unwrap().to_string();

    let user = match UserInfo::from_uuid(uuid).await {
        Ok(a) => a,
        Err(err) => {
            warn!("500/user/qq(get)->{}: {}.", ip.to_string(), err.clone());
            return HttpResponse::InternalServerError().body(err);
        }
    };

    return match user.bind_qq {
        None => {
            warn!("500/user/qq(get)->{}: User haven't bind yet.", ip.to_string());
            HttpResponse::InternalServerError().body("User haven't bind yet.")
        }
        Some(a) => {
            info!("200/user/qq(get)->{}: {}", ip.to_string(), a.to_string());
            HttpResponse::Ok().body(a.to_string())
        }
    };
}

#[get("/user/luck")]
pub async fn user_luck_get(req: HttpRequest, _req_body: String) -> impl Responder {
    let uri = req.uri().to_string();
    let uri_encoded = UrlEncodedData::from(uri.as_str());
    let ip = req.peer_addr().unwrap().ip().to_string();

    if !uri_encoded.exists("uuid") || !uri_encoded.exists("key") {
        warn!("400/user/luck(get)->{}: Missing argument(s).", ip.to_string());
        return HttpResponse::BadRequest().body("Missing argument(s).");
    }

    let key = uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("key")).unwrap().to_string();

    if examine_key(key).await.is_err() {
        warn!("401/user/luck(get)->{}: Wrong key.", ip.to_string());
        return HttpResponse::Unauthorized().body("Wrong key.");
    }

    let uuid = uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("uuid")).unwrap().to_string();

    let user = match UserInfo::from_uuid(uuid).await {
        Ok(a) => a,
        Err(err) => {
            warn!("500/user/luck(get)->{}: {}.", ip.to_string(), err.clone());
            return HttpResponse::InternalServerError().body(err);
        }
    };

    if user.bind_qq.is_none() {
        warn!("500/user/luck(get)->{}: User have not bind yet.", ip.to_string());
        return HttpResponse::InternalServerError().body("user have not bind yet.");
    }
    let luck = calc_luck(user.bind_qq.unwrap().to_string());

    info!("200/user/luck(get)->{}: {}", ip.to_string(), luck.to_string());
    HttpResponse::Ok().body(luck.to_string())
}