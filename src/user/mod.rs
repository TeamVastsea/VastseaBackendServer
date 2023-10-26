use std::collections::HashMap;

use axum::extract::Query;
use axum::http::StatusCode;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod microsoft;
pub mod xbox;
pub mod minecraft;
mod info;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInfo {
    //_id: mc uuid
    pub _id: String,
    pub display_name: String,
    pub enabled: bool,
    pub group: Vec<String>,
    pub bind_qq: Option<i64>,
    pub ban_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserMCProfile {
    pub uuid: String,
    pub user_name: String,
}

pub async fn user_get(query: Query<HashMap<String, String>>) -> Result<String, (StatusCode, String)> {
    let mut method = "";
    let mut user_info: UserInfo = UserInfo {
        _id: "".to_string(),
        display_name: "".to_string(),
        enabled: false,
        group: vec![],
        bind_qq: None,
        ban_reason: None,
    };

    if let Some(code) = query.get("code") {
        method = "code";
        let mc_profile = match UserMCProfile::from_code(code).await {
            Ok(a) => a,
            Err(err) => {
                return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Cannot get user mc profile: {err}")));
            }
        };
        user_info = match UserInfo::from_mc_profile(mc_profile).await {
            Ok(a) => a,
            Err(err) => {
                return Err((StatusCode::INTERNAL_SERVER_ERROR, err))
            }
        }
    }
    if let Some(code) = query.get("atoken") {
        method = "atoken";
        let mc_profile = match UserMCProfile::from_access_token(code).await {
            Ok(a) => a,
            Err(err) => {
                return Err((StatusCode::INTERNAL_SERVER_ERROR, err));
            }
        };
        user_info = match UserInfo::from_mc_profile(mc_profile).await {
            Ok(a) => a,
            Err(err) => {
                return Err((StatusCode::INTERNAL_SERVER_ERROR, err));
            }
        }
    }
    if let Some(token) = query.get("htoken") {
        method = "htoken";
        user_info = match UserInfo::from_token(token).await {
            Ok(a) => a,
            Err(err) => {
                return Err((StatusCode::INTERNAL_SERVER_ERROR, err));
            }
        };
    }

    if method == "" {
        return Err((StatusCode::BAD_REQUEST, String::from("missing args")));
    }

    let mut result = serde_json::to_value(user_info.clone()).unwrap();

    if let Some(a) = query.get("token") {
        if a == "true" {
            result["token"] = Value::from(user_info.to_token().await);
        }
    };

    return Ok(result.to_string());
}