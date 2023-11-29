use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub bio: String,
    pub username: String,
    pub token: String,
    pub avatar: String,
    pub collections: Vec<i32>
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub token: String,
    pub bio: String,
    pub username: String,
    pub avatar: String,
}
