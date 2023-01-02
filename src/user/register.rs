use base64::encode;
use mongodb::bson::{doc, Document};
use mongodb::{Collection};
use rand_core::OsRng;
use scrypt::Params;
use scrypt::password_hash::SaltString;
use crate::{CONFIG, MONGODB, user};
use user::UserInfo;

impl UserInfo {
    async fn register(&self) -> Result<(), String> {
        let collection: &Collection<UserInfo> = &unsafe { MONGODB.as_ref() }.unwrap().collection("users");
        collection.insert_one(self, None).await.unwrap();
        Ok(())
    }
}

pub async fn register(username: String, password: String, mail_box: String, ip: &String) -> Result<UserInfo, String> {
    let collect: &Collection<Document> = &unsafe { MONGODB.as_ref() }.unwrap().collection("users");
    if let Ok(user) = collect.find_one(doc! {"username": &username}, None).await {
        if user != None {
            return Err("User already exists".to_string());
        }
    };

    //generate salt
    let salt = SaltString::generate(&mut OsRng);
    let mut output: [u8; 4096] = [0; 4096];
    scrypt::scrypt(password.as_bytes(), salt.as_bytes(), &Params::recommended(), &mut output).unwrap();
    let user = UserInfo {
        username,
        mail_box,
        group: unsafe { CONFIG["defaultUserGroup"].as_str().unwrap().to_string().split(",").map(|s| s.to_string()).collect() },
        enabled: true,
        bind_user: "none".to_string(),
        bind_qq: -1,
        otp: false,
        password: encode(output),
        salt: salt.to_string(),
        ip: Some(ip.clone()),
    };

    user.register().await.unwrap();
    Ok(user)
}