use bson::DateTime;
use chrono::{Months};
use mongodb::Collection;
use rand::distributions::Alphanumeric;
use rand::prelude::Distribution;
use rand_core::SeedableRng;
use crate::command::ApiKey;
use crate::MONGODB;

static USAGE: &str = "key <Description>";

pub async fn get_api_key(params: Vec<String>) {
    let mut std_rng = rand::rngs::StdRng::from_entropy();

    if params.get(0).is_none() || params.get(0).unwrap() == "" {
        println!("Params expected. Usage: {}", USAGE);
        return;
    }

    let collection: &Collection<ApiKey> = &unsafe { MONGODB.as_ref() }.unwrap().collection("api_key");
    let key_string = Alphanumeric.sample_iter(&mut std_rng).take(19).map(char::from).collect::<String>();

    let nbf = DateTime::now().to_chrono();
    let nat = nbf.checked_add_months(Months::new(3)).unwrap();

    let key = ApiKey {
        _id: key_string,
        description: params.get(0).unwrap().to_string(),
        nbf,
        nat,
    };

    collection.insert_one(&key, None).await.expect("Cannot access mongodb");

    println!("Create succeeded!\nYour key is {}\nValid before {}.", key._id, key.nat.format("%Y-%m-%d %H:%M:%S"));
}