use diesel;
use diesel::prelude::*;
use schema::{chores, chore_entries};

use db::Connection;
use chore_entry::ChoreEntry;

#[table_name="chores"]
#[derive(Identifiable, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Chore {
    pub id: Option<i32>,    //Chore ID
    pub name: String,       //Chore name
    pub interval: i32,      //Interval of chore (in days)
    pub userid: i32         //User ID who created the chore
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

    pub fn read_entries(id: i32, connection: &Connection) -> Vec<ChoreEntry> {
        chore_entries::table.filter(chore_entries::choreid.eq(id))
            .order(chore_entries::id.asc())
            .load::<ChoreEntry>(&(**connection))
            .unwrap()
    }

    pub fn update(id: i32, chore: Chore, connection: &Connection) -> bool {
        diesel::update(chores::table.find(id)).set(&chore).execute(&(**connection)).is_ok()
    }

    pub fn delete(id: i32, connection: &Connection) -> bool {
        diesel::delete(chores::table.find(id)).execute(&(**connection)).is_ok()
    }
}
