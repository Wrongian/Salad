use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GetUser {
    pub username: String,
    pub id: i32,
    // hashed password
    pub password: String,
    pub email: String,
    pub bio: Option<String>,
    pub is_private: bool,
    pub salt: String,
    pub display_name: String,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertUser {
    pub username: String,
    // hashed password
    pub password: String,
    pub email: String,
    pub bio: Option<String>,
    pub is_private: bool,
    pub salt: String,
    pub display_name: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::users)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub password: Option<String>,
    pub salt: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub is_private: Option<bool>,
    pub display_name: Option<String>,
}

#[derive(Selectable, Queryable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserProfileView {
    pub id: i32,
    pub is_private: bool,
    pub username: String,
    pub bio: Option<String>,
    pub display_name: String,
}
