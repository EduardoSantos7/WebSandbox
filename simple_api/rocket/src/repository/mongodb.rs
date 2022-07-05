use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::car::Car;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};

pub struct MongoRepo {
    col: Collection<Car>,
}

// #[derive(Responder)]
// pub enum CustomerError<T> {
//     #[response(status = 400)]
//     Unauthorized(T),
//     #[response(status = 400)]
//     BadRequest(T),
//     #[response(status = 404)]
//     NotFound(T),
// }

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

    pub fn get_car(&self, id: &String) -> Result<Car, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let car_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting car's detail");
        Ok(car_detail.unwrap())
    }

    pub fn update_car(&self, id: &String, new_car: Car) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_car.id
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating car");
        Ok(updated_doc)
    }

    pub fn delete_car(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let car_detail = self
            .col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting car");
        Ok(car_detail)
    }
}
