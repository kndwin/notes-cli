#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{NewNote, Note};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn create_note<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Note {
    use schema::notes;

    let new_note = NewNote {
        title: title,
        body: body,
    };

    diesel::insert_into(notes::table)
        .values(&new_note)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn list_notes(conn: &PgConnection) -> Vec<Note> {
    use schema::notes::dsl::*;

    notes.load::<Note>(conn)
        .expect("Error loading posts")
}

pub fn delete_note<'a>(conn: &PgConnection, id_to_delete: &'a i32) -> bool {
    use schema::notes::dsl;

    diesel::delete(dsl::notes.find(id_to_delete))
        .execute(conn)
        .is_ok()
}
pub fn edit_note<'a>(
    conn: &PgConnection, 
    id_to_edit: &'a i32, 
    title: &'a str, 
    body: &'a str
) -> bool {
    use schema::notes::dsl; 

    let new_note = NewNote {
        title: title,
        body: body,
    };

    diesel::update(dsl::notes.find(id_to_edit))
        .set(&new_note)
        .execute(conn)
        .is_ok()
}
