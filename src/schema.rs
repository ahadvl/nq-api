// @generated automatically by Diesel CLI.

diesel::table! {
    app_users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    app_verify_codes (id) {
        id -> Int4,
        code -> Int4,
        email -> Text,
        status -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    app_users,
    app_verify_codes,
);
