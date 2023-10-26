use axum::extract::{Path, Query};
use axum::Json;
use bson::doc;
use bson::oid::ObjectId;
use chrono::Utc;
use futures_util::StreamExt;
use hyper::StatusCode;
use mongodb::Collection;
use mongodb::options::FindOptions;
use serde::{Deserialize, Serialize};

use crate::MONGODB;
use crate::user::UserInfo;

mod news_file;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewsInfo {
    _id: String,
    title: String,
    description: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    released: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewsCreateInfo {
    title: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewsCreateRequest {
    info: NewsCreateInfo,
    body: String,
    token: String,
}

#[derive(Deserialize)]
pub struct GetNewsQueue {
    page: u8,
    size: Option<u8>,
}

pub async fn news_get(queue: Query<GetNewsQueue>) -> Result<String, StatusCode> {
    let page = queue.page;
    let size = match queue.size {
        None => { 10 }
        Some(a) => { a }
    };
    if page < 1 || size > 15 {
        return Err(StatusCode::BAD_REQUEST);
    }
    let skip = size * (page - 1);
    let limit = size;

    let options = FindOptions::builder().sort(doc! {"released": -1}).skip(skip as u64).limit(limit as i64).build();
    let collection: Collection<NewsInfo> = MONGODB.collection("news");

    let mut news_cursor = collection.find(doc! {}, options).await.unwrap();
    let mut news = Vec::new();

    while let Some(doc) = news_cursor.next().await {
        news.push(doc.unwrap());
    }

    return Ok(serde_json::to_string(&news).unwrap());
}

pub async fn news_id_get(Path(id): Path<i32>) -> Result<String, StatusCode> {
    let collection: Collection<NewsInfo> = MONGODB.collection("news");

    let news = match collection.find_one(doc! {"_id": id}, None).await.unwrap() {
        None => { return Err(StatusCode::NOT_FOUND); }
        Some(a) => { a }
    };
    let document = news.get_body().await;
    return Ok(document);
}

pub async fn news_post(body: Json<NewsCreateRequest>) -> Result<String, StatusCode> {
    let collection: Collection<NewsInfo> = MONGODB.collection("news");
    let info = &body.info;
    let token = &body.token;
    let body = &body.body;

    let user = match UserInfo::from_token(token).await {
        Ok(a) => { a }
        Err(_) => { return Err(StatusCode::UNAUTHORIZED); }
    };

    if !user.group.contains(&"admin".to_string()) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let info = NewsInfo {
        _id: ObjectId::new().to_hex(),
        released: Utc::now(),
        title: info.title.clone(),
        description: info.description.clone(),
    };

    collection.insert_one(&info, None).await.unwrap();

    return if info.save_body(body.to_string()).await.is_err() {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    } else {
        Ok(String::new())
    };
}