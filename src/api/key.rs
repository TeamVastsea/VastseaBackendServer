use bson::doc;
use bson::DateTime;
use mongodb::Collection;
use crate::api::ApiKey;
use crate::MONGODB;

pub async fn examine_key(key: &str) -> Result<(), ()> {
    let collection: Collection<ApiKey> = MONGODB.collection("api_key");

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