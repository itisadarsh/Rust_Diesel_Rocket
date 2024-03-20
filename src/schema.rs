// @generated automatically by Diesel CLI.

diesel::table! {
    bird (id) {
        id -> Int4,
        birdname -> Varchar,
        scientific_name -> Varchar,
        commonwealth_status -> Varchar,
    }
}
