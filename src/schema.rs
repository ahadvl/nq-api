// @generated automatically by Diesel CLI.

diesel::table! {
    app_emails (id) {
        id -> Int4,
        user_id -> Int4,
        email -> Text,
        verified -> Bool,
        primary -> Bool,
        deleted -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    app_organizations_table (id) {
        id -> Int4,
        name -> Text,
        profile_image -> Nullable<Text>,
        established_date -> Timestamptz,
        national_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    app_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        token_hash -> Varchar,
        terminated -> Bool,
        terminated_by_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    app_users (id) {
        id -> Int4,
        username -> Varchar,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        birthday -> Nullable<Timestamptz>,
        profile_image -> Nullable<Text>,
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

diesel::table! {
    quran_text (id) {
        id -> Int4,
        surah_id -> Int4,
        verse_number -> Int4,
        text -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    app_emails,
    app_organizations_table,
    app_tokens,
    app_users,
    app_verify_codes,
    quran_text,
);
