use serde::Serialize;

#[derive(Serialize)]
pub struct PaginatedGetPayload<T: Serialize = GetPaginatedProfile> {
    pub profiles: Vec<T>,
    pub total_size: i64,
}

#[derive(Serialize)]
pub struct GetPaginatedProfile {
    pub username: String,
    pub img_src: Option<String>,
    pub id: i32,
    pub display_name: String,
}

pub const PER_PAGE: i64 = 8;
