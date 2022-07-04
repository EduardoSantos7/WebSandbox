use crate::{models::car::Car, repository::mongodb::MongoRepo};
use mongodb::results::InsertOneResult;
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
