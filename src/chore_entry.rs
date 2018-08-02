use diesel;
use diesel::prelude::*;
use schema::chore_entries;
use chrono::prelude::*;
use chrono::NaiveDate;

use db::Connection;
use user::User;
use chore::Chore;

#[derive(Identifiable, Associations, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[belongs_to(User, foreign_key="userid")]
#[belongs_to(Chore, foreign_key="choreid")]
#[table_name="chore_entries"]
pub struct ChoreEntry {
    pub id: Option<i32>,    //Entry ID
    pub date: NaiveDate,    //Entry date
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

    pub fn read(id: i32, connection: &Connection) -> Vec<ChoreEntry> {
        chore_entries::table.filter(chore_entries::id.eq(id))
            .order(chore_entries::id.asc())
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
