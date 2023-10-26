use amqprs::BasicProperties;
use amqprs::channel::{BasicPublishArguments, QueueBindArguments, QueueDeclareArguments};
use amqprs::connection::{Connection, OpenConnectionArguments};

use crate::CONFIG;

pub async fn github_post_receive(payload: String) {
    let args: OpenConnectionArguments = CONFIG.rabbitmq.uri.as_str().try_into().unwrap();
    let connection = Connection::open(&args).await.unwrap();

    let channel = connection.open_channel(None).await.unwrap();
    let (queue_name, _, _) = channel.queue_declare(QueueDeclareArguments::new(&CONFIG.rabbitmq.queue_name)).await.unwrap().unwrap();

    channel.queue_bind(QueueBindArguments::new(&queue_name, &CONFIG.rabbitmq.exchange_name, &CONFIG.rabbitmq.rounting_key)).await.unwrap();

    let args = BasicPublishArguments::new(&CONFIG.rabbitmq.exchange_name, &CONFIG.rabbitmq.rounting_key);
    channel.basic_publish(BasicProperties::default(), payload.into_bytes(), args).await.unwrap();

    channel.close().await.unwrap();
    connection.close().await.unwrap();
}