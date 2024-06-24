// @generated automatically by Diesel CLI.

diesel::table! {
    links (id) {
        id -> Int4,
        user_id -> Int4,
        next_id -> Nullable<Int4>,
        prev_id -> Nullable<Int4>,
        description -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        #[max_length = 255]
        href -> Varchar,
        img_src -> Nullable<Varchar>,
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
        display_name -> Varchar,
        img_src -> Nullable<Varchar>,
    }
}

diesel::joinable!(links -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    links,
    users,
);
