use std::env;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SupabaseConfig {
    pub supabase_url_storage: String,
    pub supabase_api_key: String,
}

impl Default for SupabaseConfig {
    fn default() -> Self {
        SupabaseConfig {
            supabase_url_storage: env::var("SUPABASE_URL_STORAGE").unwrap(),
            supabase_api_key: env::var("SUPABASE_API_KEY").unwrap(),
        }
    }
}
