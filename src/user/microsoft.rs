use hyper_tls::HttpsConnector;
use base64::Engine;
use serde::{Deserialize, Serialize};
use urlencoding::encode;
use hyper::{Body, Client, Request, body::HttpBody, http::HeaderValue};
use lazy_static::lazy_static;
use serde_json::{from_str, Error};
use shadow_rs::shadow;
lazy_static!{
    pub static ref CLIENT_ID: String = String::from("54473e32-df8f-42e9-a649-9419b0dab9d3");
    pub static ref SIGN_IN_URL: String = format!("https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize?client_id={}&response_type=code&redirect_uri=https%3A%2F%2Fmccteam.github.io%2Fredirect.html&scope=XboxLive.signin%20offline_access%20openid%20email&prompt=select_account&response_mode=fragment", CLIENT_ID.clone());
    pub static ref TOKEN_URL: String = String::from("https://login.microsoftonline.com/consumers/oauth2/v2.0/token");
}
#[allow(dead_code)]
pub fn get_sign_in_url_with_hint(login_hint:String)->String
{
    return SIGN_IN_URL.clone()+&String::from("&login_hint=")+&encode(login_hint.as_str()).into_owned();
}
#[derive(Clone,Debug,Deserialize, Serialize)]
pub struct LoginResponse {
    pub email: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: Option<i32>,
    pub error: Option<String>,
    pub error_description: Option<String>,
    pub id_token: Option<String>
}
#[derive(Clone,Debug,Deserialize, Serialize)]
pub struct JwtPayload
{
    pub email: String
}
#[allow(dead_code)]
pub async fn request_access_token(code: String)->Result<LoginResponse,String> {
    request_token(format!("client_id={}&grant_type=authorization_code&redirect_uri=https%3A%2F%2Fmccteam.github.io%2Fredirect.html&code={}",CLIENT_ID.clone(),code)).await
}
#[allow(dead_code)]
pub async fn refresh_access_token(refresh_token: String) -> Result<LoginResponse,String> {
    request_token(format!("client_id={}&grant_type=refresh_token&redirect_uri=https%3A%2F%2Fmccteam.github.io%2Fredirect.html&refresh_token={}",CLIENT_ID.clone(),refresh_token)).await
}
#[allow(dead_code)]
pub fn get_payload(token: String) -> Result<String,String>{
    let content=token.split('.').nth(1);
    if content.is_none() {
        return Err("cannot find content in the token".to_string());
    }
    let json_payload=String::from_utf8(decode(content.unwrap().to_string())?);
    if json_payload.is_err() {
        return Err(json_payload.err().unwrap().to_string());
    }
    return Ok(json_payload.unwrap());
}
pub fn decode(input: String) -> Result<Vec<u8>,String> {
    let mut output=input;
    output=output.replace('-', "+");
    output=output.replace('_', "/");
    match output.len()%4usize {
        0=>{},
        2=>output += "==",
        3=>output += "=",
        _=>return Err("Illegal base64url string!".to_string())
    }
    let converted=base64::engine::general_purpose::STANDARD.decode(output);
    if converted.is_err() {
        return Err(converted.err().unwrap().to_string());
    }
    return Ok(converted.unwrap());
}
pub async fn request_token(post_data: String)->Result<LoginResponse,String> {
    let https=HttpsConnector::new();
    let client=Client::builder().build::<_,hyper::Body>(https);
    let mut request_builder=Request::builder().method("POST");
    let headers=request_builder.headers_mut().unwrap();
    shadow!(build);
    let ua="VastSeaServer/".to_owned()+build::SHORT_COMMIT;
    headers.insert("User-Agent",HeaderValue::from_str(&ua).unwrap());
    headers.insert("Accept",HeaderValue::from_static("*/*"));
    headers.insert("Connection", HeaderValue::from_static("close"));
    headers.insert("Content-Type",HeaderValue::from_static("application/x-www-form-urlencoded"));
    let response=client.request(request_builder.uri(TOKEN_URL.clone()).body(Body::from(post_data)).unwrap()).await;
    if response.is_err()
    {
        return Err(response.err().unwrap().to_string());
    }
    let data=String::from_utf8(response.unwrap().into_body().data().await.unwrap().unwrap().to_vec());
    if data.is_err()
    {
        return Err(data.err().unwrap().to_string());
    }    
    let login_response:Result<LoginResponse,Error> = from_str(data.unwrap().as_str());
    if login_response.is_err() {
        return Err(login_response.err().unwrap().to_string());
    }
    let resp=login_response.unwrap();
    if resp.error.is_some() {
        return Err(resp.error_description.unwrap());
    }else{
        let payload=get_payload(resp.clone().id_token.unwrap());
        let json_payload:Result<JwtPayload,Error>=from_str(payload?.as_str());
        if json_payload.is_err(){
            return Err(json_payload.err().unwrap().to_string());
        }
        return Ok(LoginResponse{email: Some(json_payload.unwrap().email),..resp});
    }
}
