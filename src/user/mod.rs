use std::borrow::Cow;
use actix_web::{get, HttpRequest, HttpResponse, Responder};

use mongodb::bson::{doc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url_encoded_data::UrlEncodedData;


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

#[get("/users")]
pub async fn user_get(req: HttpRequest, _req_body: String) -> impl Responder {
    let uri = req.uri().to_string();
    let uri_encoded = UrlEncodedData::from(uri.as_str());
    let mut method = "";
    let mut user_info: UserInfo = UserInfo {
        _id: "".to_string(),
        display_name: "".to_string(),
        enabled: false,
        group: vec![],
        bind_qq: None,
        ban_reason: None,
    };

    if uri_encoded.keys().contains(&"code") {
        method = "code";
        let code = uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("code")).unwrap().to_string();
        let mc_profile = match UserMCProfile::from_code(code).await {
            Ok(a) => a,
            Err(err) => {
                return HttpResponse::InternalServerError().body(err);
            }
        };
        user_info = match UserInfo::from_mc_profile(mc_profile).await {
            Ok(a) => a,
            Err(err) => {
                return HttpResponse::InternalServerError().body(err);
            }
        }
    }
    if uri_encoded.keys().contains(&"atoken") {
        method = "atoken";
        let code = uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("atoken")).unwrap().to_string();
        let mc_profile = match UserMCProfile::from_access_token(code).await {
            Ok(a) => a,
            Err(err) => {
                return HttpResponse::InternalServerError().body(err);
            }
        };
        user_info = match UserInfo::from_mc_profile(mc_profile).await {
            Ok(a) => a,
            Err(err) => {
                return HttpResponse::InternalServerError().body(err);
            }
        }
    }
    if uri_encoded.keys().contains(&"htoken") {
        method = "htoken";
        let token = uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("htoken")).unwrap().to_string();
        user_info = match UserInfo::from_token(token).await {
            Ok(a) => a,
            Err(err) => {
                return HttpResponse::InternalServerError().body(err);
            }
        };
    }

    if method == "" {
        return HttpResponse::BadRequest().body("Missing args");
    }

    let mut result = serde_json::to_value(user_info.clone()).unwrap();

    if let Some(a) = uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("token")) {
        if a.to_string() == "true" {
            result["token"] = Value::from(user_info.to_token().await);
        }
    };

    return HttpResponse::Ok().body(result.to_string());
}