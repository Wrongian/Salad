// @generated automatically by Diesel CLI.

diesel::table! {
    follows (id) {
        id -> Int4,
        from_id -> Int4,
        to_id -> Int4,
    }
}

diesel::table! {
    images (id) {
        id -> Int4,
        img_src -> Varchar,
        filename -> Varchar,
        user_id -> Nullable<Int4>,
        link_id -> Nullable<Int4>,
    }
}

diesel::table! {
    links (id) {
        id -> Int4,
        user_id -> Int4,
        next_id -> Nullable<Int4>,
        description -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        #[max_length = 255]
        href -> Varchar,
    }
}

diesel::table! {
    pending_follow_requests (id) {
        id -> Int4,
        from_id -> Int4,
        to_id -> Int4,
    }
}

diesel::table! {
    reset_password_request (id) {
        id -> Int4,
        user_id -> Int4,
        created_at -> Timestamp,
        code -> Varchar,
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
    }
}

diesel::joinable!(images -> links (link_id));
diesel::joinable!(images -> users (user_id));
diesel::joinable!(links -> users (user_id));
diesel::joinable!(reset_password_request -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    follows,
    images,
    links,
    pending_follow_requests,
    reset_password_request,
    users,
);
