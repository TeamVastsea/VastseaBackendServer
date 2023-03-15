use bson::doc;
use mongodb::Collection;
use crate::MONGODB;
use crate::user::UserInfo;

pub async fn ban_user(uuid: String, reason: String) -> Result<(), ()> {
    let collection: &Collection<UserInfo> = &unsafe { MONGODB.as_ref() }.unwrap().collection("users");
    let user = collection.find_one(doc! {"_id": uuid}, None).await.unwrap();


    if user.is_none() || user.clone().unwrap().enabled != true {
        return Err(());
    }
    let user = user.unwrap();

    let collection: &Collection<UserInfo> = &unsafe { MONGODB.as_ref() }.unwrap().collection("users");
    collection.update_many(doc! {"_id": user._id.clone()}, doc! {"$set": {"enabled": false, "ban_reason": Some(reason)}}, None).await.unwrap();

    Ok(())
}

pub async fn ban_user_qq(qq: i64, reason: String) -> Result<(), ()> {
    let collection: &Collection<UserInfo> = &unsafe { MONGODB.as_ref() }.unwrap().collection("users");
    let user = collection.find_one(doc! {"bind_qq": qq}, None).await.unwrap();


    if user.is_none() || user.clone().unwrap().enabled != true {
        return Err(());
    }
    let user = user.unwrap();

    let collection: &Collection<UserInfo> = &unsafe { MONGODB.as_ref() }.unwrap().collection("users");
    collection.update_many(doc! {"_id": user._id.clone()}, doc! {"$set": {"enabled": false, "ban_reason": Some(reason)}}, None).await.unwrap();

    Ok(())
}