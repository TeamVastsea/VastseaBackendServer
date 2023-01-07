
use mongodb::bson::doc;
use mongodb::Collection;
use rand::distributions::Alphanumeric;
use rand::prelude::Distribution;
use crate::MONGODB;
use crate::user::{BindTokenInfo, UserInfo};

impl BindTokenInfo {
    pub async fn register_token(&mut self) -> String {
        let mut rng = rand::thread_rng();
        let token = Alphanumeric.sample_iter(&mut rng).take(16).map(char::from).collect::<String>().to_uppercase();
        self.token = Some(token.clone());
        let collection: &Collection<BindTokenInfo> = unsafe { &MONGODB.as_ref().unwrap().collection("bind_token") };
        collection.insert_one(self, None).await.unwrap();
        token
    }

    async fn check_token(token: String) -> Result<BindTokenInfo, String> {
        let collection: &Collection<BindTokenInfo> = unsafe { &MONGODB.as_ref().unwrap().collection("bind_token") };
        let info = match collection.find_one(doc! {"token": token.clone()}, None).await.unwrap() {
            Some(a) => a,
            None => { return Err("Token not found".to_string()); }
        };
        let elapse = match info.insert_time.to_system_time().elapsed() {
            Ok(a) => a,
            Err(e) => {
                collection.delete_many(doc! {"token": token}, None).await.unwrap();
                return Err(e.to_string());
            }
        };
        if elapse.as_secs() > 600 {
            return Err("Token expired".to_string());
        }
        Ok(info)
    }
}

pub async fn bind_qq(token: String, qq: i64) -> Result<u64, String> {
    let user_info = match BindTokenInfo::check_token(token).await {
        Ok(info) => info,
        Err(e) => { return Err(e); }
    };

    //update user
    let collection: &Collection<UserInfo> = unsafe { &MONGODB.as_ref().unwrap().collection("users") };
    if collection.find_one(doc! {"username": user_info.username.clone()}, None).await.unwrap().expect("User not found").bind_qq != -1 {
        return Err("Already Bound".to_string());
    }
    let count = collection.update_one(doc! {"username": user_info.username.clone()}, doc! {"$set": {"bind_qq": qq}}, None).await.unwrap().modified_count;
    let collection: &Collection<BindTokenInfo> = unsafe { &MONGODB.as_ref().unwrap().collection("bind_token") };
    collection.delete_many(doc! {"username": user_info.username}, None).await.unwrap();
    Ok(count)
}