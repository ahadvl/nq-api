// @generated automatically by Diesel CLI.

diesel::table! {
    quran_text (id) {
        id -> Int4,
        surah -> Int4,
        verse -> Int4,
        text -> Text,
    }
}

diesel::table! {
    app_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        token_hash -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

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

diesel::allow_tables_to_appear_in_same_query!(app_tokens, app_users, app_verify_codes, quran_text);
