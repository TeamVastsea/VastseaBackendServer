
use mongodb::bson::{doc, Document};
use mongodb::{Collection};
use crate::{MONGODB, user};
use user::UserInfo;

impl UserInfo {
    pub async fn register(&self) -> Result<(), String> {
        let collection: &Collection<UserInfo> = &unsafe { MONGODB.as_ref() }.unwrap().collection("users");
        collection.insert_one(self, None).await.unwrap();
        Ok(())
    }
    pub async fn find_uuid(uuid: String) -> Result<UserInfo, String> {
        let collection: &Collection<UserInfo> = &unsafe { MONGODB.as_ref() }.unwrap().collection("users");
        let res=collection.find_one(doc! {"uuid": uuid}, None).await.unwrap();
        res.ok_or(String::from("User not found"))
    }
}

pub async fn register(uuid: String) -> Result<UserInfo, String> {
    let collect: &Collection<Document> = &unsafe { MONGODB.as_ref() }.unwrap().collection("users");
    if let Ok(user) = collect.find_one(doc! {"uuid": &uuid}, None).await {
        if user != None {
            return Err("User already exists".to_string());
        }
    };
    /*
    //generate salt
    let salt = SaltString::generate(&mut OsRng);
    let mut output: [u8; 4096] = [0; 4096];
    scrypt::scrypt(password.as_bytes(), salt.as_bytes(), &Params::recommended(), &mut output).unwrap();
    */
    let user = UserInfo {
        uuid: uuid,
        bind_qq: -1
    };

    user.register().await.unwrap();
    Ok(user)
}
