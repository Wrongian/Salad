pub mod routes {
    pub mod auth;
    pub mod links;
    pub mod profiles;
}

// These are custom structs for handling errors and responses
pub mod types;

// these are the database modals
pub mod models {
    pub mod images;
    pub mod links;
    pub mod users;
}

// these define the helper functions used in various routes and backend logic
pub mod helpers;

// these are the testing functions we use
pub mod tests;

// these are the queries to external api like aws s3 buckets and the database
pub mod connectors {
    pub mod buckets;
    pub mod db;
}

// this the database schema
pub mod schema;