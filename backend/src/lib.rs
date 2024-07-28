pub mod routes {
    pub mod auth;
    pub mod follow;
    pub mod links;
    pub mod profiles;
    pub mod settings;
    pub mod search;
}

// These are custom structs for handling errors and responses
pub mod types;

// these are the database modals
pub mod models;

// these define the helper functions used in various routes and backend logic
pub mod helpers;

// these are the testing functions we use
pub mod tests;

// these are the queries to external api like aws s3 buckets and the database
pub mod connectors {
    pub mod buckets;
    pub mod db;
    pub mod smtp;
}

// this the database schema
pub mod schema;
