use actix_web::{HttpResponse, post, Responder, web};
use amqprs::BasicProperties;
use amqprs::channel::{BasicPublishArguments, QueueBindArguments, QueueDeclareArguments};
use amqprs::connection::{Connection, OpenConnectionArguments};
use crate::{CONFIG};

#[post("/github")]
pub async fn github_post(payload: web::Bytes) -> impl Responder {
    let args: OpenConnectionArguments = CONFIG.rabbitmq.uri.as_str().try_into().unwrap();
    let connection = Connection::open(&args).await.unwrap();

    let channel = connection.open_channel(None).await.unwrap();
    let (queue_name, _, _) = channel.queue_declare(QueueDeclareArguments::new(&CONFIG.rabbitmq.queue_name)).await.unwrap().unwrap();

    channel.queue_bind(QueueBindArguments::new(&queue_name, &CONFIG.rabbitmq.exchange_name, &CONFIG.rabbitmq.rounting_key)).await.unwrap();

    let args = BasicPublishArguments::new(&CONFIG.rabbitmq.exchange_name, &CONFIG.rabbitmq.rounting_key);
    channel.basic_publish(BasicProperties::default(), payload.to_vec(), args).await.unwrap();

    channel.close().await.unwrap();
    connection.close().await.unwrap();

    HttpResponse::Ok()
}