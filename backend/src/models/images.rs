use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::images)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GetImage {
    pub id: i32,
    pub img_src: String,
    pub filename: String,
    pub user_id: Option<i32>,
    pub link_id: Option<i32>,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::images)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertProfileImage {
    pub img_src: String,
    pub filename: String,
    pub user_id: i32,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::images)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertLinkImage {
    pub img_src: String,
    pub filename: String,
    pub link_id: i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::images)]
pub struct UpdateImage {
    pub user_id: Option<i32>,
    pub link_id: Option<i32>,
    pub filename: Option<String>,
    pub img_src: Option<String>,
}
