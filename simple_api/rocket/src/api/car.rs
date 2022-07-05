use crate::{models::car::Car, repository::mongodb::MongoRepo};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};

#[post("/car", data = "<new_car>")]
pub fn create_car(
    db: &State<MongoRepo>,
    new_car: Json<Car>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Car {
        id: None,
        region: new_car.region.to_owned(),
        price: new_car.price.to_owned(),
        year: new_car.year.to_owned(),
        manufacturer: new_car.manufacturer.to_owned(),
        model: new_car.model.to_owned(),
        condition: new_car.condition.to_owned(),
    };
    let car_detail = db.create_car(data);
    match car_detail {
        Ok(car) => Ok(Json(car)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/car/<car_id>")]
pub fn get_car_by_id(db: &State<MongoRepo>, car_id: String) -> Result<Json<Car>, Status> {
    // Get data from DB
    let car_detail = db.get_car(&car_id);

    match car_detail {
        Ok(car) => Ok(Json(car)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/car/<car_id>", data = "<new_car>")]
pub fn update_car(
    db: &State<MongoRepo>,
    car_id: String,
    new_car: Json<Car>,
) -> Result<Json<Car>, Status> {
    let id = car_id;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let data = Car {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        region: new_car.region.to_owned(),
        price: new_car.price.to_owned(),
        year: new_car.year.to_owned(),
        manufacturer: new_car.manufacturer.to_owned(),
        model: new_car.model.to_owned(),
        condition: new_car.condition.to_owned(),
    };
    let update_result = db.update_car(&id, data);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_car_info = db.get_car(&id);
                return match updated_car_info {
                    Ok(car) => Ok(Json(car)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/car/<car_id>")]
pub fn delete_car(db: &State<MongoRepo>, car_id: String) -> Result<Json<&str>, Status> {
    let id = car_id;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_car(&id);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("Car successfully deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
