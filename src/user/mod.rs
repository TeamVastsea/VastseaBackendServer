use serde::{Deserialize, Serialize};

pub mod register;
pub mod login;

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    username: String,
    mail_box: String,
    group: Vec<String>,
    enabled: bool,
    bind_user: String,
    bind_qq: i64,
    otp: bool,
    password: String,
    salt: String,
}