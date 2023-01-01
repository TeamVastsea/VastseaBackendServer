use pbkdf2::password_hash::{SaltString};
use serde::{Deserialize, Serialize, Serializer};

pub mod register;

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