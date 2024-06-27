use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Insertable, Serialize, Debug)]
#[diesel(table_name = crate::schema::links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GetLink {
    pub id: i32,
    pub user_id: i32,
    pub next_id: Option<i32>,
    pub description: Option<String>,
    pub title: Option<String>,
    pub href: String,
}

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertLink {
    pub user_id: i32,
    pub next_id: Option<i32>,
    pub description: Option<String>,
    pub title: Option<String>,
    pub href: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::links)]
pub struct UpdateLink {
    pub user_id: Option<i32>,
    pub next_id: Option<i32>,
    pub description: Option<String>,
    pub title: Option<String>,
    pub href: Option<String>,
}
