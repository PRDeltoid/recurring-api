use diesel;
use diesel::prelude::*;
use schema::users;

use db::Connection;

#[table_name="users"]
#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: Option<i32>,    //User ID
    pub username: String,   //User's name
}

impl User {
    pub fn create_user(user: User, connection: &Connection) -> User {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(&(**connection))
            .expect("Error creating new user");

        //users::table.order(users::id.desc()).first(connection).unwrap()
        user
    }
}