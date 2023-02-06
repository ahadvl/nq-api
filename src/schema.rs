// @generated automatically by Diesel CLI.

diesel::table! {
    app_accounts (id) {
        id -> Int4,
        username -> Varchar,
        account_type -> Text,
    }
}

diesel::table! {
    app_emails (id) {
        id -> Int4,
        account_id -> Int4,
        email -> Text,
        verified -> Bool,
        primary -> Bool,
        deleted -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    app_employees (id) {
        id -> Int4,
        org_account_id -> Int4,
        employee_account_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    app_organizations (id) {
        id -> Int4,
        account_id -> Int4,
        name -> Varchar,
        profile_image -> Nullable<Text>,
        established_date -> Date,
        national_id -> Varchar,
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
        account_id -> Int4,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        birthday -> Nullable<Date>,
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
    app_accounts,
    app_emails,
    app_employees,
    app_organizations,
    app_tokens,
    app_users,
    app_verify_codes,
    quran_text,
);
