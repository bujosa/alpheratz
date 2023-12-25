#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use rocket::State;

type Database = Mutex<HashMap<String, Person>>;
 
#[derive(Deserialize, Serialize, Clone, Debug)]
struct Person {
    name: String,
    lastname: String,
    email: String,
}

#[post("/", format = "json", data = "<person>")]
fn create_person(person: Json<Person>, db: &State<Database>) -> Status {
    let mut database = db.lock().expect("database lock");
    database.insert(person.email.clone(), person.into_inner());
    Status::Created
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
    rocket::build().mount("/", routes![create_person, get_all_persons, get_person]).manage(Mutex::new(HashMap::<String, Person>::new()))
}