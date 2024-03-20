use diesel::prelude::*;
use rocket::serde::Serialize;
// use crate::schema::bird;
#[derive(Queryable, Serialize)]
#[diesel(table_name = crate::schema::bird)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Bird {
    pub id: i32,
    pub birdname: String,
    pub scientific_name : String,
    pub commonwealth_status: String,
}