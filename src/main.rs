#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate rocket;
#[macro_use]
#[allow(unused)]
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
#[allow(unused_imports)]
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use rocket_contrib::{Json};

mod db;
mod user;
mod schema;

use user::User;  //User models
use db::Connection;

#[post("/", data="<user>")]
fn create(user: Json<User>, connection: Connection) -> Json<User> {
    let insert = User {id: None, ..user.into_inner()};
    Json(User::create_user(insert, &connection))
}

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = db::connect(&database_url);

    rocket::ignite()
        .manage(manager)
        .mount("/user", routes![create])
        .launch();

}
