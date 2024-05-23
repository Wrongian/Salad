// @generated automatically by Diesel CLI.

diesel::table! {
    _sqlx_migrations (version) {
        version -> Int8,
        description -> Text,
        installed_on -> Timestamptz,
        success -> Bool,
        checksum -> Bytea,
        execution_time -> Int8,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 30]
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        bio -> Nullable<Varchar>,
        is_private -> Bool,
        salt -> Varchar,
        display_name -> Varchar ,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    users,
);
