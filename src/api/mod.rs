mod bind;
mod key;
mod ban;
mod luck;

use std::collections::HashMap;
use axum::extract::Query;
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
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
    pub nat: DateTime<Utc>,
}

pub async fn user_bind_qq_patch(query: Query<HashMap<String, String>>) -> Result<String, (StatusCode, String)> {
    if !query.contains_key("uuid") || !query.contains_key("qq") || !query.contains_key("key") {
        return Err((StatusCode::BAD_REQUEST, "Missing arguments.".to_string()));
    }

    let uuid = query.get("uuid").unwrap();
    let qq = query.get("qq").unwrap();
    let qq = match qq.parse::<i64>() {
        Ok(a) => { a }
        Err(err) => {
            return Err((StatusCode::BAD_REQUEST, "Cannot parse qq: ".to_string() + err.to_string().as_str()));
        }
    };
    let key = query.get("key").unwrap();
    if examine_key(key).await.is_err() {
        return Err((StatusCode::UNAUTHORIZED, "Invalid key.".to_string()));
    }

    return if bind_qq(uuid, qq).await {
        Ok(String::new())
    } else {
        Err((StatusCode::INTERNAL_SERVER_ERROR, "User not found or already bound.".to_string()))
    };
}

pub async fn user_ban_put(query: Query<HashMap<String, String>>) -> Result<String, (StatusCode, String)> {
    if !((query.contains_key("uuid") || query.contains_key("qq")) && query.contains_key("key")) {
        return Err((StatusCode::BAD_REQUEST, "Missing arguments.".to_string()));
    }
    let key = query.get("key").unwrap();

    let reason = match query.get("reason") {
        None => { "" }
        Some(r) => { r }
    };

    if examine_key(key).await.is_err() {
        return Err((StatusCode::UNAUTHORIZED, "Invalid key.".to_string()));
    }

    return if query.contains_key("uuid") { //search by uuid
        let uuid = query.get("uuid").unwrap();
        if ban_user(uuid, reason).await {
            Ok(String::new())
        } else {
            Err((StatusCode::INTERNAL_SERVER_ERROR, "User not found or already disabled.".to_string()))
        }
    } else {
        let qq = match query.get("qq").unwrap().parse::<i64>() { //search by qq
            Ok(a) => a,
            Err(err) => {
                return Err((StatusCode::BAD_REQUEST, "Cannot parse qq: ".to_string() + err.to_string().as_str()));
            }
        };
        if ban_user_qq(qq, reason).await {
            Ok(String::new())
        } else {
            Err((StatusCode::INTERNAL_SERVER_ERROR, "User not found or already disabled.".to_string()))
        }
    };
}

pub async fn user_qq_get(query: Query<HashMap<String, String>>) -> Result<String, (StatusCode, String)> {

    if !query.contains_key("uuid") || !query.contains_key("key") {
        return Err((StatusCode::BAD_REQUEST, "Missing arguments.".to_string()));
    }

    let key = query.get("key").unwrap();

    if examine_key(key).await.is_err() {
        return Err((StatusCode::UNAUTHORIZED, "Invalid key.".to_string()));
    }

    let uuid = query.get("uuid").unwrap();

    let user = match UserInfo::from_uuid(uuid).await {
        Ok(a) => a,
        Err(err) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, err));
        }
    };

    return match user.bind_qq {
        None => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, "User have not bound yet.".to_string()))
        }
        Some(a) => {
            Ok(a.to_string())
        }
    };
}

pub async fn user_luck_get(query: Query<HashMap<String, String>>) -> Result<String, (StatusCode, String)> {

    if !query.contains_key("uuid") || !query.contains_key("key") {
        return Err((StatusCode::BAD_REQUEST, "Missing arguments.".to_string()));
    }

    let key = query.get("key").unwrap();

    if examine_key(key).await.is_err() {
        return Err((StatusCode::UNAUTHORIZED, "Invalid key.".to_string()));
    }

    let uuid = query.get("uuid").unwrap();

    let user = match UserInfo::from_uuid(uuid).await {
        Ok(a) => a,
        Err(_) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "User have not bound yet.".to_string()));
        }
    };

    if user.bind_qq.is_none() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "User have not bound yet.".to_string()));
    }
    let luck = calc_luck(user.bind_qq.unwrap().to_string());

    Ok(luck.to_string())
}