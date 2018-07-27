use diesel;
use diesel::prelude::*;
use schema::chores;

use db::Connection;

#[table_name="chores"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Chore {
    pub id: Option<i32>,    //User ID
    pub name: String,   //User's name
    pub interval: i32   //Interval of chore due
}

impl Chore {
    pub fn create(chore: Chore, connection: &Connection) -> Chore {
        diesel::insert_into(chores::table)
            .values(&chore)
            .execute(&(**connection))
            .expect("Error creating new chore");

        chores::table.order(chores::id.desc()).first(&(**connection)).unwrap()
    }
    pub fn read(connection: &Connection) -> Vec<Chore> {
        chores::table.order(chores::id.asc())
            .load::<Chore>(&(**connection))
            .unwrap()
    }

    pub fn update(id: i32, chore: Chore, connection: &Connection) -> bool {
        diesel::update(chores::table.find(id)).set(&chore).execute(&(**connection)).is_ok()
    }

    pub fn delete(id: i32, connection: &Connection) -> bool {
        diesel::delete(chores::table.find(id)).execute(&(**connection)).is_ok()
    }
}