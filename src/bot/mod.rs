mod bind;
mod key;

use std::borrow::Cow;
use actix_web::{HttpRequest, HttpResponse, patch, Responder};
use simple_log::{info, warn};
use url_encoded_data::UrlEncodedData;
use crate::bot::bind::bind_qq;
use crate::bot::key::examine_key;

#[patch("/user")]
pub async fn user_patch(req: HttpRequest, _req_body: String) -> impl Responder {
    let uri = req.uri().to_string();
    let uri_encoded = UrlEncodedData::from(uri.as_str());
    let ip = req.peer_addr().unwrap().ip().to_string();

    if !uri_encoded.exists("name") || !uri_encoded.exists("qq") || !uri_encoded.exists("key") {
        warn!("400/user(patch)->{}: Missing argument(s).", ip.to_string());
        return HttpResponse::BadRequest().body("Missing argument(s).");
    }

    let display_name = uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("name")).unwrap().to_string();
    let qq = uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("qq")).unwrap().to_string();
    let qq = match qq.parse::<i64>() {
        Ok(a) => {a}
        Err(err) => {
            warn!("400/user(patch)->{}: Missing argument(s).", ip.to_string());
            return HttpResponse::BadRequest().body("Cannot parse qq: ".to_string() + err.to_string().as_str());
        }
    };
    let key = uri_encoded.as_map_of_single_key_to_first_occurrence_value().get(&Cow::from("key")).unwrap().to_string();
    if examine_key(key).await.is_err() {
        warn!("401/user(patch)->{}: Wrong key.", ip.to_string());
        return HttpResponse::Unauthorized().body("Wrong key.");
    }

    return match bind_qq(display_name, qq).await {
        Ok(_) => {
            info!("200/user(patch)->{}", ip.to_string());
            HttpResponse::Ok().body("")
        }
        Err(_) => {

            warn!("500/user(patch)->{}: User not found or already bound.", ip.to_string());
            HttpResponse::InternalServerError().body("User not found or already bound.")
        }
    };
}