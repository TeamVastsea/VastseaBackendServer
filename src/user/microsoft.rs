use hyper_tls::HttpsConnector;
use base64::Engine;
use serde::{Deserialize, Serialize};
use hyper::{Body, Client, Request, body, http::HeaderValue};
use lazy_static::lazy_static;
use serde_json::{from_str, Error};
use shadow_rs::shadow;
use urlencoding::encode;
use crate::CONFIG;
lazy_static! {
    pub static ref REDIRECT_URL: String = encode(&CONFIG.oauth.redirect_url).into_owned();
    pub static ref SIGN_IN_URL: String = format!("https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize?client_id={}&response_type=code&redirect_uri={}&scope=XboxLive.signin%20offline_access%20openid%20email&prompt=select_account&response_mode=fragment", CONFIG.oauth.client_id, REDIRECT_URL.clone());
    pub static ref TOKEN_URL: String = String::from("https://login.microsoftonline.com/consumers/oauth2/v2.0/token");
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub email: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: Option<i32>,
    pub error: Option<String>,
    pub error_description: Option<String>,
    pub id_token: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JwtPayload
{
    pub email: String,
}

#[allow(dead_code)]
pub async fn request_access_token(code: String) -> Result<LoginResponse, String> {
    request_token(format!("client_id={}&grant_type=authorization_code&redirect_uri={}&code={}", CONFIG.oauth.client_id, REDIRECT_URL.clone(), code)).await
}

#[allow(dead_code)]
pub async fn refresh_access_token(refresh_token: String) -> Result<LoginResponse, String> {
    request_token(format!("client_id={}&grant_type=refresh_token&redirect_uri={}&refresh_token={}", CONFIG.oauth.client_id, REDIRECT_URL.clone(), refresh_token)).await
}

#[allow(dead_code)]
pub fn get_payload(token: String) -> Result<String, String> {
    let content = token.split('.').nth(1);
    if content.is_none() {
        return Err("cannot find content in the token".to_string());
    }
    let json_payload = String::from_utf8(decode(content.unwrap().to_string())?);
    if json_payload.is_err() {
        return Err(json_payload.err().unwrap().to_string());
    }
    return Ok(json_payload.unwrap());
}

pub fn decode(input: String) -> Result<Vec<u8>, String> {
    let mut output = input;
    output = output.replace('-', "+");
    output = output.replace('_', "/");
    match output.len() % 4usize {
        0 => {}
        2 => output += "==",
        3 => output += "=",
        _ => return Err("Illegal base64url string!".to_string())
    }
    return match base64::engine::general_purpose::STANDARD.decode(output) {
        Err(e) => Err(e.to_string()),
        Ok(a) => Ok(a),
    };
}

pub async fn request_token(post_data: String) -> Result<LoginResponse, String> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);
    let mut request_builder = Request::builder().method("POST");
    let headers = request_builder.headers_mut().unwrap();
    shadow!(build);
    let ua = "VastSeaServer/".to_owned() + build::SHORT_COMMIT;
    headers.insert("User-Agent", HeaderValue::from_str(&ua).unwrap());
    headers.insert("Accept", HeaderValue::from_static("*/*"));
    headers.insert("Connection", HeaderValue::from_static("close"));
    headers.insert("Content-Type", HeaderValue::from_static("application/x-www-form-urlencoded"));
    let mut response = match client.request(request_builder.uri(TOKEN_URL.clone()).body(Body::from(post_data)).unwrap()).await {
        Err(e) => { return Err(e.to_string()); }
        Ok(a) => a,
    };
    let all_data = match body::to_bytes(response.body_mut()).await {
        Err(e) => { return Err("cannot read response\n".to_owned() + &e.to_string()); }
        Ok(a) => a.to_vec(),
    };
    let data = match String::from_utf8(all_data) {
        Err(e) => { return Err(e.to_string()); }
        Ok(a) => a,
    };
    let login_response: LoginResponse = match from_str(data.as_str()) {
        Err(e) => { return Err(e.to_string()); }
        Ok(a) => a,
    };


    if login_response.error.is_some() {
        Err(login_response.error_description.unwrap())
    } else {
        let payload = get_payload(login_response.clone().id_token.unwrap());
        let json_payload: Result<JwtPayload, Error> = from_str(payload?.as_str());
        if json_payload.is_err() {
            return Err(json_payload.err().unwrap().to_string());
        }
        Ok(LoginResponse { email: Some(json_payload.unwrap().email), ..login_response })
    }
}
