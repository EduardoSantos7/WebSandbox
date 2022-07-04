//add the modules
mod api; 
mod models;
mod repository;


#[macro_use]
extern crate rocket;


use rocket::http::Status;
use rocket::Request;

//add imports below
use api::car::create_car;
use repository::mongodb::MongoRepo;

#[derive(Responder)]
enum Error<T> {
    #[response(status = 400)]
    Unauthorized(T),
    #[response(status = 400)]
    BadRequest(T),
    #[response(status = 404)]
    NotFound(T),
}

#[get("/<car_id>")]
fn get_car_by_id(car_id: &str) -> Result<Status, Error<&'static str>> {
    // if car_id.is_empty() {
    //     return Err(Error::BadRequest("car_id must not be empty"));
    // }
    // Get data from DB

    Ok(Status::Accepted)
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![create_car])
        .register("/", catchers![not_found])
}
