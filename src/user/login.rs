use base64::{decode, encode};
use mongodb::bson::{doc, Document};
use mongodb::Collection;
use scrypt::{Params, scrypt};
use crate::{CONFIG, MONGODB};
use crate::user::{TokenInfo, UserInfo};
use chrono::{Local};
use jwt_simple::prelude::{Claims, Duration, HS256Key, MACLike};

impl UserInfo {
    fn check_password(&self, password: &String) -> bool {
        let mut output: [u8; 4096] = [0; 4096];
        scrypt(password.as_bytes(), self.salt.as_bytes(), &Params::default(), &mut output).unwrap();
        self.password == encode(output)
    }
    fn get_token(&self, info: TokenInfo) -> String {
        let key = HS256Key::from_bytes(decode(unsafe { CONFIG["tokenKey"].as_str().unwrap() }).unwrap().as_slice());
        let claim = Claims::with_custom_claims(info, Duration::from_days(7));
        key.authenticate(claim).unwrap()
    }
    async fn log_login(&self, ip: &String, login_type: &str) {
        let collection: &Collection<Document> = &unsafe { MONGODB.as_ref() }.unwrap().collection("login_logs");
        collection.insert_one(doc! {"username": &self.username, "time": &Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), "type": login_type, "ip": ip}, None).await.unwrap();
    }
}

pub async fn login_password(username: String, password: String, ip: String) -> Result<(UserInfo, String), String> {
    let collect = &unsafe { MONGODB.as_ref() }.unwrap().collection("users");
    let user: UserInfo = match collect.find_one(doc! {"username": &username}, None).await {
        Ok(Some(a)) => a,
        _ => return Err("Can not find user".to_string())
    };

    if !user.check_password(&password) {
        return Err("Wrong password".to_string());
    }

    if !user.enabled {
        return Err("User not enabled".to_string());
    }

    user.log_login(&ip, "password").await;
    Ok((user.clone(), user.get_token(TokenInfo {
        username,
        salt: user.salt.clone(),
        ip,
    })))
}

pub async fn login_token(token: &str, ip: &str) -> Result<UserInfo, ()> {
    let token_info = verify_token(token.to_string());
    let token_info = match token_info {
        Err(_) => { return Err(()); }
        Ok(a) => a
    };
    if token_info.ip.as_str() != ip {
        return Err(());
    }
    let collect = &unsafe { MONGODB.as_ref() }.unwrap().collection("users");
    let user: UserInfo = match collect.find_one(doc! {"username": &token_info.username, "salt": &token_info.salt}, None).await {
        Ok(Some(a)) => UserInfo {
            password: "Protected".to_string(),
            ..a
        },
        _ => { return Err(()); }
    };
    Ok(user)
}

//Only confirm if it is a valid token, but do not promise it is legal for use
fn verify_token(token: String) -> Result<TokenInfo, ()> {
    let key = HS256Key::from_bytes(decode(unsafe { CONFIG["tokenKey"].as_str().unwrap() }).unwrap().as_slice());
    let claims = key.verify_token::<TokenInfo>(&token, None);
    match claims {
        Ok(info) => Ok(info.custom),
        Err(_) => Err(())
    }
}