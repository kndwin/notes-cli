#[derive(Queryable)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

use super::schema::notes;

#[derive(Insertable, AsChangeset)]
#[table_name="notes"]
pub struct NewNote<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
