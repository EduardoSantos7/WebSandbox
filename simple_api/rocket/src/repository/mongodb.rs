use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::car::Car;
use mongodb::{
    bson::extjson::de::Error,
    results::InsertOneResult,
    sync::{Client, Collection},
};

pub struct MongoRepo {
    col: Collection<Car>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<Car> = db.collection("Cars");
        MongoRepo { col }
    }

    pub fn create_car(&self, new_car: Car) -> Result<InsertOneResult, Error> {
        let new_doc = Car {
            id: None,
            region: new_car.region,
            price: new_car.price,
            year: new_car.year,
            manufacturer: new_car.manufacturer,
            model: new_car.model,
            condition: new_car.condition,
        };
        let car = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating car");
        Ok(car)
    }
}
