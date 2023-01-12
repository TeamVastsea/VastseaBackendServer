use core::lazy;

use base64::Engine;
use serde::{Deserialize, Serialize};
use urlencoding::encode;
use hyper::{Body, Client, Request, body::HttpBody, http::HeaderValue};
use lazy_static::lazy_static;
use serde_json::{from_str, Error};
lazy_static!{
    pub static ref AUTHORIZE: String=String::from("https://login.live.com/oauth20_authorize.srf?client_id=000000004C12AE6F&redirect_uri=https://login.live.com/oauth20_desktop.srf&scope=service::user.auth.xboxlive.com::MBI_SSL&display=touch&response_type=token&locale=en");
    
}