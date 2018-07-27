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
use rocket_contrib::{Json, Value};

mod db;
mod user;
mod chore;
mod schema;


mod users {
    use rocket_contrib::{Json, Value};
    use user::User;  //User models
    use db::Connection;
    #[post("/", data = "<user>")]
    fn create(user: Json<User>, connection: Connection) -> Json<User> {
        let insert = User { id: None, ..user.into_inner() };
        Json(User::create(insert, &connection))
    }

    #[get("/")]
    fn read(connection: Connection) -> Json<Value> {
        Json(json!(User::read(&connection)))
    }

    #[put("/<id>", data = "<user>")]
    fn update(id: i32, user: Json<User>, connection: Connection) -> Json<Value> {
        let update = User { id: Some(id), ..user.into_inner() };
        Json(json!({
        "success": User::update(id,  update, &connection)
    }))
    }

    #[delete("/<id>")]
    fn delete(id: i32, connection: Connection) -> Json<Value> {
        Json(json!({
        "success": User::delete(id, &connection)
    }))
    }
}

mod chores {
    use rocket_contrib::{Json, Value};
    use chore::Chore;  //Chore model
    use db::Connection;
    #[post("/", data = "<chore>")]
    fn create(chore: Json<Chore>, connection: Connection) -> Json<Chore> {
        let insert = Chore { id: None, ..chore.into_inner() };
        Json(Chore::create(insert, &connection))
    }

    #[get("/")]
    fn read(connection: Connection) -> Json<Value> {
        Json(json!(Chore::read(&connection)))
    }

    #[put("/<id>", data = "<chore>")]
    fn update(id: i32, chore: Json<Chore>, connection: Connection) -> Json<Value> {
        let update = Chore { id: Some(id), ..chore.into_inner() };
        Json(json!({
        "success": Chore::update(id,  update, &connection)
    }))
    }

    #[delete("/<id>")]
    fn delete(id: i32, connection: Connection) -> Json<Value> {
        Json(json!({
        "success": Chore::delete(id, &connection)
    }))
    }
}

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = db::connect(&database_url);

    rocket::ignite()
        .manage(manager)
        .mount("/user", routes![users::create,  users::update, users::delete])
        .mount("/users", routes![users::read])
        .mount("/chore", routes![chores::create, chores::update, chores::delete])
        .mount("/chores", routes![chores::read])
        .launch();
}
