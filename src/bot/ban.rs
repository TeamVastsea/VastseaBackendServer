use bson::doc;
use mongodb::Collection;
use crate::MONGODB;
use crate::user::UserInfo;

pub async fn ban_user(name: String, reason: String) -> Result<(), ()> {
    let collection: &Collection<UserInfo> = &unsafe { MONGODB.as_ref() }.unwrap().collection("users");
    let user = collection.find_one(doc! {"display_name": name}, None).await.unwrap();


    if user.is_none() || user.clone().unwrap().enabled != true {
        return Err(());
    }
    let user = user.unwrap();

    let collection: &Collection<UserInfo> = &unsafe { MONGODB.as_ref() }.unwrap().collection("users");
    collection.update_many(doc! {"_id": user._id.clone()}, doc! {"$set": {"enabled": false, "ban_reason": Some(reason)}}, None).await.unwrap();

    Ok(())
}