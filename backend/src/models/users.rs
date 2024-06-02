use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub username: String,
    // hashed password
    pub password: String,
    pub email: String,
    pub bio: Option<String>,
    pub is_private: bool,
    pub salt: String,
    pub display_name: String,
}

pub struct UserProfileView {
    pub username: String,
    pub bio: Option<String>,
    pub display_name: String,
    pub picture: String,
}
