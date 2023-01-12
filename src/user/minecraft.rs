use serde::{Deserialize, Serialize};
use simple_log::debug;
use hyper::{Body, Client, Request, body::HttpBody, http::HeaderValue};
use lazy_static::lazy_static;
use serde_json::{from_str};
lazy_static!{
    pub static ref LOGIN_WITH_XBOX:String="https://api.minecraftservices.com/authentication/login_with_xbox".to_string();
    pub static ref OWNERSHIP:String="https://api.minecraftservices.com/entitlements/mcstore".to_string();
    pub static ref PROFILE:String="https://api.minecraftservices.com/minecraft/profile".to_string();
}
pub async fn login_with_xbox(userHash:String,xstsToken:String)->Result<String,String>
{
    let client=Client::new();
    let mut request_builder=Request::builder().method("POST");
    let headers=request_builder.headers_mut().unwrap();
    headers.insert("Accept",HeaderValue::from_static("application/json"));
    headers.insert("Connection", HeaderValue::from_static("close"));
    headers.insert("Content-Type",HeaderValue::from_static("application/json"));
    let postData="{\"identityToken\": \"XBL3.0 x=".to_owned() + &userHash + ";" + &xstsToken + "\"}";
    let response=client.request(request_builder.uri(LOGIN_WITH_XBOX.clone()).body(Body::from(postData)).unwrap()).await;
    if response.is_err()
    {
        return Err(response.err().unwrap().to_string());
    }
    let mut resp=response.unwrap();
    let data=String::from_utf8(resp.body_mut().data().await.unwrap().unwrap().to_vec());
    if data.is_err()
    {
        return Err(data.err().unwrap().to_string());
    }
    let data_unwrap=data.unwrap();
    debug!("{}",data_unwrap);
    let json:AccessToken=from_str(data_unwrap.as_str()).unwrap();
    return Ok(json.access_token);
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessToken{
    pub access_token: String
}
pub async fn user_has_game(accessToken:String)->Result<bool,String>
{
    let client=Client::new();
    let mut request_builder=Request::builder().method("GET");
    let headers=request_builder.headers_mut().unwrap();
    headers.insert("Accept",HeaderValue::from_static("*/*"));
    headers.insert("Connection", HeaderValue::from_static("close"));
    headers.insert("Authorization",HeaderValue::from_str(format!("Bearer {}",accessToken).as_str()).unwrap());
    let response=client.request(request_builder.uri(OWNERSHIP.clone()).body(Body::empty()).unwrap()).await;
    if response.is_err()
    {
        return Err(response.err().unwrap().to_string());
    }
    let mut resp=response.unwrap();
    let data=String::from_utf8(resp.body_mut().data().await.unwrap().unwrap().to_vec());
    if data.is_err()
    {
        return Err(data.err().unwrap().to_string());
    }
    let data_unwrap=data.unwrap();
    debug!("{}",data_unwrap);
    let json:Items=from_str(data_unwrap.as_str()).unwrap();
    return Ok(json.items.len()>0);
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Items{
    pub items: Vec<Item>
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item{

}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserProfile{
    pub UUID:String,
    pub UserName:String
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserProfileResponse{
    pub id:String,
    pub name:String
}
pub async fn get_user_profile(accessToken:String)->Result<UserProfile,String>
{
    let client=Client::new();
    let mut request_builder=Request::builder().method("GET");
    let headers=request_builder.headers_mut().unwrap();
    headers.insert("Accept",HeaderValue::from_static("*/*"));
    headers.insert("Connection", HeaderValue::from_static("close"));
    headers.insert("Authorization",HeaderValue::from_str(format!("Bearer {}",accessToken).as_str()).unwrap());
    let response=client.request(request_builder.uri(OWNERSHIP.clone()).body(Body::empty()).unwrap()).await;
    if response.is_err()
    {
        return Err(response.err().unwrap().to_string());
    }
    let mut resp=response.unwrap();
    let data=String::from_utf8(resp.body_mut().data().await.unwrap().unwrap().to_vec());
    if data.is_err()
    {
        return Err(data.err().unwrap().to_string());
    }
    let data_unwrap=data.unwrap();
    debug!("{}",data_unwrap);
    let json:UserProfileResponse=from_str(data_unwrap.as_str()).unwrap();
    return Ok(UserProfile{UUID:json.id,UserName:json.name});
}
