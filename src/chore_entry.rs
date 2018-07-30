use diesel;
use diesel::prelude::*;
use schema::chore_entries;
use chrono::prelude::*;
use chrono::NaiveDate;

use db::Connection;

#[table_name="chore_entries"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct ChoreEntry {
    pub id: Option<i32>,    //Entry ID
    pub date: NaiveDate,          //Entry date
    pub userid: i32,        //User ID who created entry
    pub choreid: i32        //Chore ID of parent chore
}

impl ChoreEntry {
    pub fn create(entry: ChoreEntry, connection: &Connection) -> ChoreEntry {
        diesel::insert_into(chore_entries::table)
            .values(&entry)
            .execute(&(**connection))
            .expect("Error creating new entry");

        chore_entries::table.order(chore_entries::id.desc()).first(&(**connection)).unwrap()
    }
    pub fn read(connection: &Connection) -> Vec<ChoreEntry> {
        chore_entries::table.order(chore_entries::id.asc())
            .load::<ChoreEntry>(&(**connection))
            .unwrap()
    }

    pub fn update(id: i32, entry: ChoreEntry, connection: &Connection) -> bool {
        diesel::update(chore_entries::table.find(id)).set(&entry).execute(&(**connection)).is_ok()
    }

    pub fn delete(id: i32, connection: &Connection) -> bool {
        diesel::delete(chore_entries::table.find(id)).execute(&(**connection)).is_ok()
    }
}
