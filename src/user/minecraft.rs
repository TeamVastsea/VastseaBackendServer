use hyper_tls::HttpsConnector;

use hyper::{Body, Client, Request, body, http::HeaderValue};
use serde_json::{from_str, Value};
use crate::user::UserMCProfile;

pub static LOGIN_WITH_XBOX: &str = "https://api.minecraftservices.com/authentication/login_with_xbox";
pub static OWNERSHIP: &str = "https://api.minecraftservices.com/entitlements/mcstore";
pub static PROFILE: &str = "https://api.minecraftservices.com/minecraft/profile";

pub async fn login_with_xbox(user_hash: String, xsts_token: String) -> Result<String, String>
{
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);
    let mut request_builder = Request::builder().method("POST");
    let headers = request_builder.headers_mut().unwrap();
    headers.insert("Accept", HeaderValue::from_static("application/json"));
    headers.insert("Connection", HeaderValue::from_static("close"));
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    let post_data = "{\"identityToken\": \"XBL3.0 x=".to_owned() + &user_hash + ";" + &xsts_token + "\"}";
    let mut response = match client.request(request_builder.uri(LOGIN_WITH_XBOX.clone()).body(Body::from(post_data)).unwrap()).await {
        Err(e) => { return Err(e.to_string()); }
        Ok(a) => a,
    };
    let all_data = match body::to_bytes(response.body_mut()).await {
        Err(e) => { return Err(e.to_string()); }
        Ok(a) => a.to_vec(),
    };
    let data = match String::from_utf8(all_data) {
        Err(e) => { return Err(e.to_string()); }
        Ok(a) => a,
    };
    let json: Value = from_str(data.as_str()).unwrap();
    return match json["access_token"].as_str() {
        Some(a) => Ok(a.to_string()),
        None => Err("Token is null.".to_string()),
    };
}


pub async fn user_has_game(access_token: String) -> Result<bool, String>
{
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);
    let mut request_builder = Request::builder().method("GET");
    let headers = request_builder.headers_mut().unwrap();
    headers.insert("Accept", HeaderValue::from_static("*/*"));
    headers.insert("Connection", HeaderValue::from_static("close"));
    headers.insert("Authorization", HeaderValue::from_str(format!("Bearer {}", access_token).as_str()).unwrap());
    let mut response = match client.request(request_builder.uri(OWNERSHIP.clone()).body(Body::empty()).unwrap()).await {
        Err(e) => { return Err(e.to_string()); }
        Ok(a) => a,
    };
    let all_data = match body::to_bytes(response.body_mut()).await {
        Err(e) => { return Err(e.to_string()); }
        Ok(a) => a.to_vec(),
    };
    let data = match String::from_utf8(all_data) {
        Err(e) => { return Err(e.to_string()); }
        Ok(a) => a,
    };
    let json: Value = from_str(data.as_str()).unwrap();
    let items = match json["items"].as_array() {
        Some(a) => a,
        None => { return Err(format!("Cannot parse return value({})", data.as_str())); }
    };
    return Ok(!items.is_empty());
}

pub async fn get_user_profile(access_token: String) -> Result<UserMCProfile, String>
{
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);
    let mut request_builder = Request::builder().method("GET");
    let headers = request_builder.headers_mut().unwrap();
    headers.insert("Accept", HeaderValue::from_static("*/*"));
    headers.insert("Connection", HeaderValue::from_static("close"));
    headers.insert("Authorization", HeaderValue::from_str(format!("Bearer {}", access_token).as_str()).unwrap());
    let mut response = match client.request(request_builder.uri(PROFILE.clone()).body(Body::empty()).unwrap()).await {
        Err(e) => { return Err(e.to_string()); }
        Ok(a) => a,
    };
    let all_data = match body::to_bytes(response.body_mut()).await {
        Err(e) => { return Err(e.to_string()); }
        Ok(a) => a.to_vec(),
    };
    let data = match String::from_utf8(all_data) {
        Err(e) => { return Err(e.to_string()); }
        Ok(a) => a,
    };
    let json: Value = from_str(data.as_str()).unwrap();
    let uuid = match json["id"].as_str() {
        Some(a) => a.to_string(),
        None => { return Err("Cannot get uuid".to_string()); }
    };
    let user_name = match json["name"].as_str() {
        Some(a) => a.to_string(),
        None => { return Err("Cannot get user name".to_string()); }
    };
    return Ok(UserMCProfile { uuid, user_name });
}
