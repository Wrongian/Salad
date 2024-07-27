use serde::Serialize;

#[derive(Serialize)]
pub struct PaginatedGetPayload {
    pub profiles: Vec<GetPaginatedProfile>,
    pub total_size: i64,
}

#[derive(Serialize)]
pub struct GetPaginatedProfile {
    pub username: String,
    pub img_src: Option<String>,
    pub id: i32,
    pub display_name: String,
}

pub static PER_PAGE: i64 = 8;
