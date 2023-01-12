
use mongodb::bson::doc;
use mongodb::Collection;
use crate::MONGODB;
use crate::user::{UserInfo};

pub async fn bind_qq(uuid: String, qq: i64) -> Result<u64, String> {
    let user_info = match UserInfo::find_uuid(uuid.clone()).await {
        Ok(info) => info,
        Err(e) => { return Err(e); }
    };

    //update user
    if user_info.bind_qq != -1 {
        return Err("Already Bound".to_string());
    }
    let collection: &Collection<UserInfo> = &unsafe { MONGODB.as_ref() }.unwrap().collection("users");
    Ok(collection.update_one(doc!{"uuid":uuid}, doc!{"$set":{"bind_qq":qq}}, None).await.unwrap().modified_count)
}
