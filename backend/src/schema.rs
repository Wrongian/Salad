// @generated automatically by Diesel CLI.

diesel::table! {
    links (link_id) {
        link_id -> Int4,
        user_id -> Int4,
        description -> Nullable<Text>,
        #[max_length = 255]
        href -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 30]
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        bio -> Nullable<Text>,
        is_private -> Bool,
        salt -> Varchar,
        display_name -> Varchar,
    }
}

diesel::joinable!(links -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    links,
    users,
);
