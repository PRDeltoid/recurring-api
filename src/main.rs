#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::{Json, Value};

//Chore Type. This is the generic chore information, not associated with any dates
#[derive(Serialize, Deserialize)]
pub struct Chore {
    pub id: Option<u32>,    //Chore ID
    pub name: String,       //User's name for the chore
    pub interval_time: u8,  //The time (in days) before a chore is due again
    pub userid: u32,     //ID of the user who created the chore schedule. Foreign key.
    //Description?
    //Tags?
}

//Actual instances of a due chore. This only keeps track of what chore it is and when it was done.
#[derive(Serialize, Deserialize)]
pub struct ChoreEntry {
    pub id: Option<u32>,    //ChoreEntry ID
    pub choreid: u32,       //ID of the chore type. Foreign key.
    pub userid: u32,        //ID of the user who completed the chore. Foreign key.
    pub date: String,       //Date that the chore was completed on
}

//Keeps track of user information
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<u32>,    //User ID
    pub name: String,       //User's name
}

///Chore endpoints
#[post("/", data="<chore>")]
fn create_chore(chore: Json<Chore>) -> Json<Chore> {
    chore
}

#[get("/<choreid>")]
fn read_chore(choreid: u32) -> Json<Value> {
    Json(json!({"id": choreid, "name": "chore1"}))
}

#[get("/")]
fn read_chores() -> Json<Value> {
    Json(json!([{"name": "chore1"},
                {"name": "chore2"}]))

}

#[put("/<choreid>", data="<chore>")]
fn update_chore(choreid: u32, chore: Json<Chore>) -> Json<Chore> {
    chore
}

#[delete("/<choreid>")]
fn delete_chore(choreid: u32) -> Json<Value> {
    Json(json!({"status": "ok"}))
}


fn main() {
    rocket::ignite()
        .mount("/chore", routes![create_chore, read_chore, update_chore, delete_chore])
        .mount("/chores", routes![read_chores])
        .launch();
}
