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
        account_id -> Int4,
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
    mushafs (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        source -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    quran_ayahs (id) {
        id -> Int4,
        surah_id -> Int4,
        ayah_number -> Int4,
        sajdeh -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    quran_surahs (id) {
        id -> Int4,
        name -> Varchar,
        period -> Nullable<Varchar>,
        number -> Int4,
        bismillah_status -> Varchar,
        bismillah_text -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    quran_words (id) {
        id -> Int4,
        ayah_id -> Int4,
        word -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    translations (id) {
        id -> Int4,
        translator_id -> Int4,
        language -> Varchar,
        release_year -> Nullable<Date>,
        source -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    translations_text (id) {
        id -> Int4,
        translation_id -> Int4,
        ayah_id -> Int4,
        text -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(app_employees -> app_accounts (employee_account_id));
diesel::joinable!(app_organizations -> app_accounts (account_id));
diesel::joinable!(app_tokens -> app_accounts (terminated_by_id));
diesel::joinable!(quran_ayahs -> quran_surahs (surah_id));
diesel::joinable!(quran_words -> quran_ayahs (ayah_id));
diesel::joinable!(translations_text -> quran_ayahs (ayah_id));
diesel::joinable!(translations_text -> translations (translation_id));

diesel::allow_tables_to_appear_in_same_query!(
    app_accounts,
    app_emails,
    app_employees,
    app_organizations,
    app_tokens,
    app_users,
    app_verify_codes,
    mushafs,
    quran_ayahs,
    quran_surahs,
    quran_words,
    translations,
    translations_text,
);
