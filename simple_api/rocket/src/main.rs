//add the modules
mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use rocket::Request;
use std::env;

//add imports below
use api::car::{create_car, delete_car, get_car_by_id, update_car};
use repository::mongodb::MongoRepo;

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[launch]
fn rocket() -> _ {
    env::set_var("RUST_BACKTRACE", "1");
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount(
            "/",
            routes![create_car, get_car_by_id, update_car, delete_car],
        )
        .register("/", catchers![not_found])
}
