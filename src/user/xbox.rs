
use hyper_tls::HttpsConnector;
use std::{collections::HashMap};

use regex::Regex;
use serde::{Deserialize, Serialize};
use simple_log::debug;
use urlencoding::{encode, decode};
use hyper::{Body, Client, Request, body::HttpBody, http::HeaderValue};
use lazy_static::lazy_static;
use serde_json::{from_str};

use super::microsoft::LoginResponse;
lazy_static!{
    pub static ref AUTHORIZE: String=String::from("https://login.live.com/oauth20_authorize.srf?client_id=000000004C12AE6F&redirect_uri=https://login.live.com/oauth20_desktop.srf&scope=service::user.auth.xboxlive.com::MBI_SSL&display=touch&response_type=token&locale=en");
    pub static ref XBL:String=String::from("https://user.auth.xboxlive.com/user/authenticate");
    pub static ref XSTS:String=String::from("https://xsts.auth.xboxlive.com/xsts/authorize");
    pub static ref USERAGENT:String=String::from("Mozilla/5.0 (XboxReplay; XboxLiveAuth/3.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/71.0.3578.98 Safari/537.36");
    pub static ref PPFT:Regex=Regex::new("sFTTag:'.*value=\"(.*)\"/>'").unwrap();
    pub static ref URL_POST:Regex=Regex::new("urlPost:'(.+?(?=\'))").unwrap();
    pub static ref CONFIRM:Regex=Regex::new("identity/confirm").unwrap();
    pub static ref INVALID_ACCOUNT:Regex=Regex::new("(?i)Sign in to").unwrap();
    pub static ref TWO_FA:Regex=Regex::new("(?i)Help us protect your account").unwrap();
    pub static ref SIGN_IN_URL:String=AUTHORIZE.clone();
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PreAuthResponse {
    pub url_post: String,
    pub ppft: String,
    pub cookies: String
}
pub async fn pre_auth()->Result<PreAuthResponse,String> {
    let https=HttpsConnector::new();
    let client=Client::builder().build::<_,hyper::Body>(https);
    let mut request_builder=Request::builder().method("GET");
    let headers=request_builder.headers_mut().unwrap();
    let ua=USERAGENT.clone();
    headers.insert("User-Agent",HeaderValue::from_str(&ua).unwrap());
    headers.insert("Accept",HeaderValue::from_static("*/*"));
    headers.insert("Connection", HeaderValue::from_static("close"));
    let response=client.request(request_builder.uri(AUTHORIZE.clone()).body(Body::empty()).unwrap()).await;
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
    let data_str=data.unwrap();
    let ppft_=PPFT.captures(data_str.as_str());
    if ppft_.is_none() {
        return Err("Fail to extract PPFT".to_string());
    }
    let ppft_=&ppft_.unwrap()[1];
    let url_post=URL_POST.captures(data_str.as_str());
    if url_post.is_none() {
        return Err("Fail to extract urlPost".to_string());
    }
    let url_post=&url_post.unwrap()[1];
    let tmp=HeaderValue::from_static("");
    Ok(PreAuthResponse { url_post: url_post.to_string(), ppft: ppft_.to_string(), cookies: resp.headers().get(hyper::header::SET_COOKIE).or_else(||Some(&tmp)).unwrap().to_str().unwrap().to_string() })
}
pub fn parse_query_string(query:String)->HashMap<String,String>
{
    let mut res=HashMap::new();
    for i in query.split("&")
    {
        res.insert(i.split('=').nth(0).unwrap().to_string(), decode(i.split('=').nth(1).unwrap()).unwrap().to_string());
    }
    return res;
}
pub async fn user_login(email:String,password:String,pre_auth:PreAuthResponse)->Result<LoginResponse,String>
{
    let https=HttpsConnector::new();
    let client=Client::builder().build::<_,hyper::Body>(https);
    let mut request_builder=Request::builder().method("POST");
    let headers=request_builder.headers_mut().unwrap();
    let ua=USERAGENT.clone();
    headers.insert(hyper::header::COOKIE, HeaderValue::from_str(&pre_auth.cookies).unwrap());
    headers.insert("User-Agent",HeaderValue::from_str(&ua).unwrap());
    headers.insert("Accept",HeaderValue::from_static("*/*"));
    headers.insert("Connection", HeaderValue::from_static("close"));
    headers.insert("Content-Type",HeaderValue::from_static("application/x-www-form-urlencoded"));
    let post_data="login=".to_owned() + &encode(&email).into_owned()
    + "&loginfmt=" + &encode(&email).into_owned()
    + "&passwd=" + &encode(&password).into_owned()
    + "&PPFT=" + &encode(&pre_auth.ppft).into_owned();
    let response=client.request(request_builder.uri(pre_auth.url_post).body(Body::from(post_data)).unwrap()).await;
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
    if resp.status().as_u16()>=300 && resp.status().as_u16()<=399
    {
        let url=resp.headers().get("Location");
        if url.is_none() {
            return Err("cannot get url".to_string());
        }
        let url=url.unwrap().to_str().unwrap().to_string();
        let url2=url.clone();
        let hash=url2.split('#').nth(1);
        let https2=HttpsConnector::new();
    let client2=Client::builder().build::<_,hyper::Body>(https2);
    let mut request_builder2=Request::builder().method("GET");
    let headers2=request_builder2.headers_mut().unwrap();
    let ua2=USERAGENT.clone();
    headers2.insert("User-Agent",HeaderValue::from_str(&ua2).unwrap());
    headers2.insert("Accept",HeaderValue::from_static("*/*"));
    headers2.insert("Connection", HeaderValue::from_static("close"));
    let response2=client2.request(request_builder2.uri(url).body(Body::empty()).unwrap()).await;
    if response2.is_err()
    {
        return Err(response2.err().unwrap().to_string());
    }
    let mut resp2=response2.unwrap();
    let data2=String::from_utf8(resp2.body_mut().data().await.unwrap().unwrap().to_vec());
    if data2.is_err()
    {
        return Err(data2.err().unwrap().to_string());
    }
    if resp2.status().as_u16()!=200
    {
        return Err("Authentication failed".to_string());
    }
    if hash.is_none()||hash.unwrap().is_empty()
    {
        return Err("Cannot extract access token".to_string());
    }
    let dict=parse_query_string(hash.unwrap().to_string());
    return Ok(LoginResponse{email:Some(email),access_token:Some(dict["access_token"].clone()),refresh_token:Some(dict["refresh_token"].clone()),expires_in:Some(dict["expires_in"].parse().unwrap()), error: None, error_description: None, id_token: None });
    }else{
        if TWO_FA.is_match(&data_unwrap.as_str())
        {
            return Err("2FA enabled but not supported yet. Use browser sign-in method or try to disable 2FA in Microsoft account settings".to_string());
        }else if INVALID_ACCOUNT.is_match(&data_unwrap.as_str())
        {
            return Err("Invalid credentials. Check your credentials".to_string());
        }else{
            return Err("Unexpected response. Check your credentials. Response code: ".to_owned()+&resp.status().as_u16().to_string());
        }
    }
}
pub async fn xbl_authenticate(login_response:LoginResponse,browser:bool)->Result<AuthenticateResponse,String>
{
    let https=HttpsConnector::new();
    let client=Client::builder().build::<_,hyper::Body>(https);
    let mut request_builder=Request::builder().method("POST");
    let headers=request_builder.headers_mut().unwrap();
    let ua=USERAGENT.clone();
    headers.insert("User-Agent",HeaderValue::from_str(&ua).unwrap());
    headers.insert("Accept",HeaderValue::from_static("application/json"));
    headers.insert("Connection", HeaderValue::from_static("close"));
    headers.insert("Content-Type",HeaderValue::from_static("application/json"));
    headers.insert("x-xbl-contract-version",HeaderValue::from_static("0"));
    let access_token=login_response.access_token;
    if access_token.is_none() {
        return Err("accessToken cannot be null".to_string());
    }
    let mut access_token=access_token.unwrap();
    if browser {
        access_token = "d=".to_owned() + &access_token;
    }
    let post_data="{".to_owned()
    + "\"Properties\": {"
    + "\"AuthMethod\": \"RPS\","
    + "\"SiteName\": \"user.auth.xboxlive.com\","
    + "\"RpsTicket\": \"" + &access_token + "\""
    + "},"
    + "\"RelyingParty\": \"http://auth.xboxlive.com\","
    + "\"TokenType\": \"JWT\""
    + "}";
    let response=client.request(request_builder.uri(XBL.clone()).body(Body::from(post_data)).unwrap()).await;
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
    if resp.status().as_u16()==200 {
        let json:XBLJson=from_str(data_unwrap.as_str()).unwrap();
        return Ok(AuthenticateResponse { resp_type:"xbl".to_string(),token: json.Token, user_hash: json.DisplayClaims.xui[0].clone().uhs });
    }else{
        return Err("XBL Authentication failed".to_string());
    }
}
pub async fn xsts_authenticate(xbl_response:AuthenticateResponse)->Result<AuthenticateResponse,String>
{
    if xbl_response.resp_type != "xbl" {
        return Err("arg xblResponse type mismatch".to_string());
    }
    let https=HttpsConnector::new();
    let client=Client::builder().build::<_,hyper::Body>(https);
    let mut request_builder=Request::builder().method("POST");
    let headers=request_builder.headers_mut().unwrap();
    let ua=USERAGENT.clone();
    headers.insert("User-Agent",HeaderValue::from_str(&ua).unwrap());
    headers.insert("Accept",HeaderValue::from_static("application/json"));
    headers.insert("Connection", HeaderValue::from_static("close"));
    headers.insert("Content-Type",HeaderValue::from_static("application/json"));
    headers.insert("x-xbl-contract-version",HeaderValue::from_static("1"));
    let post_data="{".to_owned()
    + "\"Properties\": {"
    + "\"SandboxId\": \"RETAIL\","
    + "\"UserTokens\": ["
    + "\"" + &xbl_response.token + "\""
    + "]"
    + "},"
    + "\"RelyingParty\": \"rp://api.minecraftservices.com/\","
    + "\"TokenType\": \"JWT\""
    + "}";
    let response=client.request(request_builder.uri(XBL.clone()).body(Body::from(post_data)).unwrap()).await;
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
    if resp.status().as_u16()==200 {
        let json:XBLJson=from_str(data_unwrap.as_str()).unwrap();
        return Ok(AuthenticateResponse {resp_type:"xsts".to_string(), token: json.Token, user_hash: json.DisplayClaims.xui[0].clone().uhs });
    }else{
        if resp.status().as_u16()==401 {
            let json:XErr=from_str(data_unwrap.as_str()).unwrap();
            if json.XErr==2148916233 {
                return Err("The account doesn't have an Xbox account".to_string());
            }else if json.XErr==2148916238 {
                return Err("The account is a child (under 18) and cannot proceed unless the account is added to a Family by an adult".to_string());
            }else{
                return Err("Unknown XSTS error code: ".to_owned() + &json.XErr.to_string());
            }
        }else{
            return Err("XSTS Authentication failed".to_string());
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticateResponse{
    pub resp_type: String,
    pub token:String,
    pub user_hash:String
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XBLJson{
    pub Token: String,
    pub DisplayClaims:DisplayClaims
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisplayClaims{
    pub xui: Vec<XUI>
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XUI{
    pub uhs: String
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XErr{
    XErr: u64
}