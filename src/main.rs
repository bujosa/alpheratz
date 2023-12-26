#[macro_use]
extern crate rocket;
pub use alphe::{validate_email, validate_fields, add_updated_at_field};
use rocket::serde::json::Json;
use rocket::{State};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;


type CustomResponse = Result<rocket::response::status::Custom<Json<String>>, rocket::response::status::Custom<Json<Vec<String>>>>;

type Database = Mutex<HashMap<String, Person>>;

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Person {
    name: String,
    lastname: String,
    email: String,
    updated_at: Option<String>,
}

#[validate_fields]
#[post("/", format = "json", data = "<person>")]
fn create_person(person: Json<Person>, db: &State<Database>) -> CustomResponse {
    let mut database = db.lock().expect("database lock");
    database.insert(person.email.clone(), person.into_inner());
    println!("{:?}", database);
    Ok(rocket::response::status::Custom(
        rocket::http::Status::Created,
        Json(String::from("Person created"))
    ))
}

#[validate_fields]
#[add_updated_at_field]
#[put("/<email>", format = "json", data = "<person>")]
fn update_person(email: String, mut person: Json<Person>, db: &State<Database>) -> CustomResponse {
    let mut database = db.lock().expect("database lock");
    database.insert(email, person.into_inner());
    Ok(rocket::response::status::Custom(
        rocket::http::Status::Ok,
        Json(String::from("Person updated"))
    ))
}

#[get("/")]
fn get_all_persons(db: &State<Database>) -> Json<HashMap<String, Person>> {
    let database = db.lock().expect("database lock");
    Json(database.clone())
}

#[get("/<email>")]
fn get_person(email: String, db: &State<Database>) -> Option<Json<Person>> {
    let database = db.lock().expect("database lock");
    database.get(&email).map(|person| Json(person.clone()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![create_person, get_all_persons, get_person, update_person])
        .manage(Mutex::new(HashMap::<String, Person>::new()))
}
