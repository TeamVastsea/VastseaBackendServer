mod news_file;

use actix_web::{get, HttpResponse, post, Responder, web};
use actix_web::web::Query;
use bson::doc;
use bson::oid::ObjectId;
use chrono::Utc;
use futures_util::StreamExt;
use mongodb::Collection;
use mongodb::options::FindOptions;
use serde::{Deserialize, Serialize};
use crate::MONGODB;
use crate::user::UserInfo;

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

#[get("/news")]
pub async fn news_get(queue: Query<GetNewsQueue>) -> impl Responder {
    let page = queue.page;
    let size = match queue.size {
        None => { 10 }
        Some(a) => { a }
    };
    if page < 1 || size > 15 {
        return HttpResponse::BadRequest().body("Page or size out of range.");
    }
    let skip = size * (page - 1);
    let limit = size;

    let options = FindOptions::builder().sort(doc! {"released": -1}).skip(skip as u64).limit(limit as i64).build();
    let collection: &Collection<NewsInfo> = &unsafe { MONGODB.as_ref() }.unwrap().collection("news");

    let mut news_cursor = collection.find(doc! {}, options).await.unwrap();
    let mut news = Vec::new();

    while let Some(doc) = news_cursor.next().await {
        news.push(doc.unwrap());
    }

    return HttpResponse::Ok().body(serde_json::to_string(&news).unwrap());
}

#[get("/news/{id}")]
pub async fn news_details(path: web::Path<String>) -> impl Responder {
    let collection: &Collection<NewsInfo> = &unsafe { MONGODB.as_ref() }.unwrap().collection("news");

    let news = match collection.find_one(doc! {"_id": path.into_inner()}, None).await.unwrap() {
        None => { return HttpResponse::NotFound().body("ID not found"); }
        Some(a) => { a }
    };
    let document = news.get_body().await;
    return HttpResponse::Ok().body(document);
}

#[post("/news")]
pub async fn news_create(body: web::Json<NewsCreateRequest>) -> impl Responder {
    let collection: &Collection<NewsInfo> = &unsafe { MONGODB.as_ref() }.unwrap().collection("news");
    let info = &body.info;
    let token = &body.token;
    let body = &body.body;

    let user = match UserInfo::from_token(token.to_string()).await {
        Ok(a) => { a }
        Err(_) => { return HttpResponse::Unauthorized(); }
    };

    if !user.group.contains(&"admin".to_string()) {
        return HttpResponse::Unauthorized();
    }

    let info = NewsInfo {
        _id: ObjectId::new().to_hex(),
        released: Utc::now(),
        title: info.title.clone(),
        description: info.description.clone(),
    };

    collection.insert_one(&info, None).await.unwrap();

    return if info.save_body(body.to_string()).await.is_err() {
        HttpResponse::InternalServerError()
    } else {
        HttpResponse::Ok()
    };
}