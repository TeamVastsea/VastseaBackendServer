use bson::doc;
use bson::DateTime;
use mongodb::Collection;
use simple_log::debug;
use crate::command::ApiKey;
use crate::MONGODB;

pub async fn examine_key(key: String) -> Result<(), ()> {
    let collection: &Collection<ApiKey> = &unsafe { MONGODB.as_ref() }.unwrap().collection("api_key");

    debug!("{}", key);

    let a = collection.find_one(doc! {"_id": key}, None).await.unwrap();
    if a.is_none() {
        return Err(());
    }
    let a = a.unwrap();

    if DateTime::now() < a.nbf.into() || DateTime::now() > a.nat.into() {
        return Err(());
    }

    Ok(())
}