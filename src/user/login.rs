use base64::encode;
use mongodb::bson::{doc};
use scrypt::{Params, scrypt};
use crate::MONGODB;
use crate::user::UserInfo;

impl UserInfo {
    fn check_password(&self, password: String) -> bool {
        let mut output: [u8; 4096] = [0; 4096];
        scrypt(password.as_bytes(), self.salt.as_bytes(), &Params::default(), &mut output).unwrap();
        self.password == encode(output)
    }
    fn get_token(&self) -> String {
        todo!()
    }
}

pub async fn login_password(username: String, password: String) -> Result<String, String> {
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

    Ok(user.get_token())
}