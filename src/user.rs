use diesel;
use diesel::prelude::*;
use schema::{chores, users};

use db::Connection;
use chore::Chore;

#[table_name="users"]
#[derive(Identifiable, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct User {
    pub id: Option<i32>,    //User ID
    pub username: String,   //User's name
}

impl User {
    pub fn create(user: User, connection: &Connection) -> User {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(&(**connection))
            .expect("Error creating new user");

        users::table.order(users::id.desc()).first(&(**connection)).unwrap()
    }

    pub fn read(connection: &Connection) -> Vec<User> {
        users::table.order(users::id.asc())
            .load::<User>(&(**connection))
            .unwrap()
    }

    pub fn read_user(id: i32, connection: &Connection) -> User {
        users::table.filter(users::id.eq(id))
            .order(users::id.asc())
            .first(&(**connection))
            .expect("Error pulling user")
    }

    pub fn read_chores(id: i32, connection: &Connection) -> Vec<Chore> {
        chores::table.filter(chores::userid.eq(id))
            .order(chores::id.asc())
            .load::<Chore>(&(**connection))
            .unwrap()
    }

    pub fn update(id: i32, user: User, connection: &Connection) -> bool {
        diesel::update(users::table.find(id)).set(&user).execute(&(**connection)).is_ok()
    }

    pub fn delete(id: i32, connection: &Connection) -> bool {
        diesel::delete(users::table.find(id)).execute(&(**connection)).is_ok()
    }
}