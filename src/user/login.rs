use base64::encode;
use mongodb::bson::{doc, Document};
use mongodb::Collection;
use scrypt::{Params, scrypt};
use crate::MONGODB;
use crate::user::UserInfo;
use chrono::{Local};

impl UserInfo {
    fn check_password(&self, password: String) -> bool {
        let mut output: [u8; 4096] = [0; 4096];
        scrypt(password.as_bytes(), self.salt.as_bytes(), &Params::default(), &mut output).unwrap();
        self.password == encode(output)
    }
    fn get_token(&self) -> String {
        todo!()
    }
    async fn log_login(&self, ip: String, login_type: String) {
        let collection: &Collection<Document> = &unsafe {MONGODB.as_ref()}.unwrap().collection("login_logs");
        collection.insert_one(doc! {"username": &self.username, "time": &Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), "type": login_type, "ip": ip}, None).await.unwrap();
    }
}

pub async fn login_password(username: String, password: String, ip: String) -> Result<String, String> {
    let collect = &unsafe { MONGODB.as_ref() }.unwrap().collection("users");
    let user: UserInfo = match collect.find_one(doc! {"username": &username}, None).await {
        Ok(Some(a)) => a,
        _ => return Err("Can not find user".to_string())
    };
    if !user.enabled {
        return Err("User not enabled".to_string());
    }

    if !user.check_password(password) {
        return Err("Wrong password".to_string());
    }

    user.log_login(ip, "password".to_string()).await;
    Ok(user.get_token())
}