#[macro_use]
extern crate rocket;

use diesel::prelude::*;
use rocket::{Build, Rocket};
use rocket::serde::json::Json;

use self::models::*;
use self::schema::bird::dsl::*;


mod database;
mod models;
mod schema;
#[get("/")]
fn index() -> Json<Vec<Bird>> {
    let connection = &mut database::establish_connection();
    bird.load::<Bird>(connection).map(Json).expect("Error loading birds")
}


#[launch]
fn rocket() -> Rocket<Build> {
    
    rocket::build().configure(rocket::Config::figment().merge(("port", 9797))).
    mount("/", routes![index])
}
// fn main() {
//     let port_number = 8080; // Specify the port number you want to use

//     rocket::ignite()
//         .port(port_number)
//         .mount("/", routes![index])
//         .launch();
// }