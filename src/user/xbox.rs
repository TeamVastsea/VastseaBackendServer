use hyper_tls::HttpsConnector;
use std::{collections::HashMap};

use regex::Regex;
use serde::{Deserialize, Serialize};
use hyper::{Body, Client, Request, body, http::HeaderValue};
use lazy_static::lazy_static;
use serde_json::{from_str};
lazy_static! {
    pub static ref AUTHORIZE: String=String::from("https://login.live.com/oauth20_authorize.srf?client_id=000000004C12AE6F&redirect_uri=https://login.live.com/oauth20_desktop.srf&scope=service::user.auth.xboxlive.com::MBI_SSL&display=touch&response_type=token&locale=en");
    pub static ref XBL: String=String::from("https://user.auth.xboxlive.com/user/authenticate");
    pub static ref XSTS: String=String::from("https://xsts.auth.xboxlive.com/xsts/authorize");
    pub static ref USERAGENT: String=String::from("Mozilla/5.0 (XboxReplay; XboxLiveAuth/3.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/71.0.3578.98 Safari/537.36");
    pub static ref PPFT: Regex=Regex::new("sFTTag:'.*value=\"(.*)\"/>'").unwrap();
    pub static ref URL_POST: pcre2::bytes::Regex=pcre2::bytes::RegexBuilder::new().utf(true).ucp(true).crlf(true).jit_if_available(true).build("urlPost:'(.+?(?=\'))").unwrap();
    pub static ref CONFIRM: Regex=Regex::new("identity/confirm").unwrap();
    pub static ref INVALID_ACCOUNT: Regex=Regex::new("(?i)Sign in to").unwrap();
    pub static ref TWO_FA: Regex=Regex::new("(?i)Help us protect your account").unwrap();
    pub static ref SIGN_IN_URL: String=AUTHORIZE.clone();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PreAuthResponse {
    pub url_post: String,
    pub ppft: String,
    pub cookies: HashMap<String, Vec<String>>,
}

// pub async fn pre_auth() -> Result<PreAuthResponse, String> {
//     let https = HttpsConnector::new();
//     let client = Client::builder().build::<_, Body>(https);
//     let mut request_builder = Request::builder().method("GET");
//     let headers = request_builder.headers_mut().unwrap();
//     let ua = USERAGENT.clone();
//     headers.insert("User-Agent", HeaderValue::from_str(&ua).unwrap());
//     headers.insert("Accept", HeaderValue::from_static("*/*"));
//     headers.insert("Connection", HeaderValue::from_static("close"));
//     let mut response = match client.request(request_builder.uri(AUTHORIZE.clone()).body(Body::empty()).unwrap()).await {
//         Err(e) => { return Err(e.to_string()); }
//         Ok(a) => a,
//     };
//     let all_data = match body::to_bytes(response.body_mut()).await {
//         Err(e) => { return Err(e.to_string()); }
//         Ok(a) => a.to_vec(),
//     };
//     let data = match String::from_utf8(all_data) {
//         Err(e) => { return Err(e.to_string()); }
//         Ok(a) => a,
//     };
//     let ppft_ = match PPFT.captures(data.as_str()) {
//         None => { return Err("Not fount!".to_string()); }
//         Some(a) => a,
//     };
//     let ppft_ = &ppft_[1];
//     let url_post = match URL_POST.find(data.as_str().as_bytes()) {
//         Err(_) => { return Err("Fail to extract urlPost".to_string()); }
//         Ok(a) => match a {
//             None => { return Err("Fail to extract urlPost".to_string()); }
//             Some(a) => a,
//         },
//     };
//     let url_post = data[url_post.start()..url_post.end()][9..].to_string();
//     Ok(PreAuthResponse { url_post: url_post.to_string(), ppft: ppft_.to_string(), cookies: parse_set_cookie(response.headers().get_all(hyper::header::SET_COOKIE).into_iter().map(|i| { i.to_str().unwrap().to_string() }).collect()) })
// }

// pub fn parse_query_string(query: String) -> HashMap<String, String>
// {
//     let mut res = HashMap::new();
//     for i in query.split("&")
//     {
//         res.insert(i.split('=').nth(0).unwrap().to_string(), decode(i.split('=').nth(1).unwrap()).unwrap().to_string());
//     }
//     return res;
// }

// pub fn parse_set_cookie(header_value: Vec<String>) -> HashMap<String, Vec<String>>
// {
//     let mut cookies = HashMap::new();
//     for value in header_value {
//         let mut cookie = value.split(';');
//         let mut tmp = cookie.nth(0).unwrap().split('=');
//         let options: Vec<&str> = cookie.collect();
//         let cname = tmp.nth(0).unwrap();
//         let mut cvalue = "".to_owned();
//         while tmp.clone().count() > 0 {
//             cvalue += (tmp.nth(0).unwrap().to_string() + "=").as_str();
//         }
//         cvalue = cvalue[0..cvalue.len() - 1].to_string();
//         let mut is_expired = false;
//         for option in options {
//             let mut tmp2 = option.trim().split('=');
//             if tmp2.clone().count() >= 2 {
//                 let mut tmp3 = Vec::new();
//                 tmp3.push(tmp2.nth(0).unwrap());
//                 let mut combined = "".to_owned();
//                 while tmp2.clone().count() > 0 {
//                     combined += (tmp2.nth(0).unwrap().to_string() + "=").as_str();
//                 }
//                 combined = combined[0..combined.len() - 1].to_string();
//                 tmp3.push(combined.as_str());
//                 let tmp2 = tmp3;
//                 let opt_name = tmp2[0].trim().to_lowercase();
//                 let opt_value = tmp2[1].trim().to_string();
//                 match opt_name.as_str() {
//                     "expires" => {
//                         if let Ok(exp_date) = chrono::DateTime::parse_from_rfc2822(opt_value.as_str())
//                         {
//                             if exp_date < Utc::now() {
//                                 is_expired = true;
//                             }
//                         }
//                     }
//                     "max-age" => {
//                         if let Ok(exp_int) = i32::from_str(opt_value.as_str()) {
//                             if exp_int <= 0 {
//                                 is_expired = true;
//                             }
//                         }
//                     }
//                     _ => {}
//                 }
//             }
//             if is_expired {
//                 break;
//             }
//         }
//         if !is_expired {
//             let mut cookies_tmp = cookies.clone();
//             let mut empty_vec = Vec::new();
//             let val = cookies_tmp.get_mut(cname).unwrap_or(&mut empty_vec);
//             val.push(cvalue);
//             cookies.insert(cname.to_string(), val.to_vec());
//         }
//     }
//     return cookies;
// }

// pub fn get_cookie_string(cookies: HashMap<String, Vec<String>>) -> String
// {
//     let mut result: String = "".to_string();
//     for entry in cookies {
//         let mut val = "".to_string();
//         for v in entry.1 {
//             val += &(v + &",");
//         }
//         val = val[0..val.len() - 1].to_string();
//         result += format!("{}={}; ", entry.0, val).as_str();
//     }
//     return result[0..result.len() - 2].to_string();
// }

// pub async fn user_login(email:String,password:String,pre_auth:PreAuthResponse)->Result<LoginResponse,String>
// {
//     let https=HttpsConnector::new();
//     let client=Client::builder().build::<_,hyper::Body>(https);
//     let mut request_builder=Request::builder().method("POST");
//     let headers=request_builder.headers_mut().unwrap();
//     let ua=USERAGENT.clone();
//     headers.insert(hyper::header::COOKIE, HeaderValue::from_str(&get_cookie_string(pre_auth.cookies)).unwrap());
//     headers.insert("User-Agent",HeaderValue::from_str(&ua).unwrap());
//     headers.insert("Accept",HeaderValue::from_static("*/*"));
//     headers.insert("Connection", HeaderValue::from_static("keep-alive"));
//     headers.insert("Content-Type",HeaderValue::from_static("application/x-www-form-urlencoded"));
//     //debug!("{:?}",headers);
//     let post_data="i13=0&login=".to_owned() + &encode(&email).into_owned()
//     + "&loginfmt=" + &encode(&email).into_owned()
//     + "&type=11&LoginOptions=3&lrt=&lrtPartition=&hisRegion=&hisScaleUnit=&passwd=" + &encode(&password).into_owned()
//     + "&ps=2&psRNGCDefaultType=&psRNGCEntropy=&psRNGCSLK=&canary=&ctx=&hpgrequestid=&PPFT=" + &encode(&pre_auth.ppft).into_owned()
//     + "&PPSX="+&"PassportRN"[0..rand::thread_rng().gen_range::<usize,_>(2..=10)]
//     + "&NewUser=1&FoundMSAs=&fspost=0&i21=0&CookieDisclosure=0&IsFidoSupported=1&isSignupPost=0&isRecoveryAttemptPost=0&i19="+&rand::thread_rng().gen_range::<u32,_>(1000..=9999).to_string();
//     let response=client.request(request_builder.uri(pre_auth.url_post).body(Body::from(post_data)).unwrap()).await;
//     if response.is_err()
//     {
//         return Err(response.err().unwrap().to_string());
//     }
//     let mut resp=response.unwrap();
//     let all_data=body::to_bytes(resp.body_mut()).await;
//     if let Err(err)=all_data {
//         return Err("cannot read response\n".to_owned()+&err.to_string());
//     }
//     let all_data=all_data.unwrap().to_vec();
//     let data=String::from_utf8(all_data);
//     if data.is_err()
//     {
//         return Err(data.err().unwrap().to_string());
//     }
//     let data_unwrap=data.unwrap();
//     debug!("{}",data_unwrap);
//     if resp.status().as_u16()>=300 && resp.status().as_u16()<=399
//     {
//         let url=resp.headers().get("Location");
//         if url.is_none() {
//             return Err("cannot get url".to_string());
//         }
//         let url=url.unwrap().to_str().unwrap().to_string();
//         let url2=url.clone();
//         let hash=url2.split('#').nth(1);
//         let https2=HttpsConnector::new();
//     let client2=Client::builder().build::<_,hyper::Body>(https2);
//     let mut request_builder2=Request::builder().method("GET");
//     let headers2=request_builder2.headers_mut().unwrap();
//     let ua2=USERAGENT.clone();
//     headers2.insert("User-Agent",HeaderValue::from_str(&ua2).unwrap());
//     headers2.insert("Accept",HeaderValue::from_static("*/*"));
//     headers2.insert("Connection", HeaderValue::from_static("close"));
//     let response2=client2.request(request_builder2.uri(url).body(Body::empty()).unwrap()).await;
//     if response2.is_err()
//     {
//         return Err(response2.err().unwrap().to_string());
//     }
//     let mut resp2=response2.unwrap();
//     let bytes2=body::to_bytes(resp2.body_mut()).await;
//     if let Err(err)=bytes2 {
//         return Err(err.to_string());
//     }
//     let data2=String::from_utf8(bytes2.unwrap().to_vec());
//     if data2.is_err()
//     {
//         return Err(data2.err().unwrap().to_string());
//     }
//     if resp2.status().as_u16()!=200
//     {
//         return Err("Authentication failed".to_string());
//     }
//     if hash.is_none()||hash.unwrap().is_empty()
//     {
//         return Err("Cannot extract access token".to_string());
//     }
//     let dict=parse_query_string(hash.unwrap().to_string());
//     return Ok(LoginResponse{email:Some(email),access_token:Some(dict["access_token"].clone()),refresh_token:Some(dict["refresh_token"].clone()),expires_in:Some(dict["expires_in"].parse().unwrap()), error: None, error_description: None, id_token: None });
//     }else{
//         if TWO_FA.is_match(&data_unwrap.as_str())
//         {
//             return Err("2FA enabled but not supported yet. Use browser sign-in method or try to disable 2FA in Microsoft account settings".to_string());
//         }else if INVALID_ACCOUNT.is_match(&data_unwrap.as_str())
//         {
//             return Err("Invalid credentials. Check your credentials".to_string());
//         }else{
//             return Err("Unexpected response. Check your credentials. Response code: ".to_owned()+&resp.status().as_u16().to_string());
//         }
//     }
// }

pub async fn xbl_authenticate(mut access_token: String, browser: bool) -> Result<AuthenticateResponse, String> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);
    let mut request_builder = Request::builder().method("POST");
    let headers = request_builder.headers_mut().unwrap();
    let ua = USERAGENT.clone();
    headers.insert("User-Agent", HeaderValue::from_str(&ua).unwrap());
    headers.insert("Accept", HeaderValue::from_static("application/json"));
    headers.insert("Connection", HeaderValue::from_static("close"));
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    headers.insert("x-xbl-contract-version", HeaderValue::from_static("0"));

    if browser {
        access_token = "d=".to_owned() + &access_token;
    }
    let post_data = format!(r#"{{
            "Properties": {{
                "AuthMethod": "RPS",
                "SiteName": "user.auth.xboxlive.com",
                "RpsTicket": "{}"
            }},
            "RelyingParty": "http://auth.xboxlive.com",
            "TokenType": "JWT"
        }}"#, access_token);


    let mut response = match client.request(request_builder.uri(XBL.clone()).body(Body::from(post_data)).unwrap()).await {
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
    return if response.status().as_u16() == 200 {
        let json: XBLJson = from_str(data.as_str()).unwrap();
        Ok(AuthenticateResponse { resp_type: "xbl".to_string(), token: json.Token, user_hash: json.DisplayClaims.xui[0].clone().uhs })
    } else {
        Err(response.status().to_string())
    };
}


pub async fn xsts_authenticate(xbl_response: AuthenticateResponse) -> Result<AuthenticateResponse, String>
{
    if xbl_response.resp_type != "xbl" {
        return Err("arg xblResponse type mismatch".to_string());
    }
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);
    let mut request_builder = Request::builder().method("POST");
    let headers = request_builder.headers_mut().unwrap();
    let ua = USERAGENT.clone();
    headers.insert("User-Agent", HeaderValue::from_str(&ua).unwrap());
    headers.insert("Accept", HeaderValue::from_static("application/json"));
    headers.insert("Connection", HeaderValue::from_static("close"));
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    headers.insert("x-xbl-contract-version", HeaderValue::from_static("1"));
    let post_data = format!(r#"{{
        "Properties": {{
            "SandboxId": "RETAIL",
            "UserTokens": ["{}"],
        }},
        "RelyingParty": "rp://api.minecraftservices.com/",
        "TokenType": "JWT"
    }}"#, &xbl_response.token);
    let mut response = match client.request(request_builder.uri(XSTS.clone()).body(Body::from(post_data)).unwrap()).await {
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
    return if response.status().as_u16() == 200 {
        let json: XBLJson = from_str(data.as_str()).unwrap();
        Ok(AuthenticateResponse { resp_type: "xsts".to_string(), token: json.Token, user_hash: json.DisplayClaims.xui[0].clone().uhs })
    } else {
        if response.status().as_u16() == 401 {
            let json: XErr = from_str(data.as_str()).unwrap();
            if json.XErr == 2148916233 {
                Err("The account doesn't have an Xbox account".to_string())
            } else if json.XErr == 2148916238 {
                Err("The account is a child (under 18) and cannot proceed unless the account is added to a Family by an adult".to_string())
            } else {
                Err("Unknown XSTS error code: ".to_owned() + &json.XErr.to_string())
            }
        } else {
            Err("XSTS Authentication failed".to_string())
        }
    };
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticateResponse {
    pub resp_type: String,
    pub token: String,
    pub user_hash: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XBLJson {
    pub Token: String,
    pub DisplayClaims: DisplayClaims,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisplayClaims {
    pub xui: Vec<XUI>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XUI {
    pub uhs: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XErr {
    XErr: u64,
}
