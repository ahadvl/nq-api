// @generated automatically by Diesel CLI.

diesel::table! {
    app_logs (id) {
        id -> Int4,
        status -> Nullable<Int4>,
        source_ip -> Nullable<Varchar>,
        method -> Nullable<Varchar>,
        controller -> Nullable<Varchar>,
        action -> Nullable<Varchar>,
        requested_id -> Nullable<Int4>,
        origin -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    app_tokens (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        token -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    app_users (id) {
        id -> Int4,
        username -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    app_verify_codes (id) {
        id -> Int4,
        status -> Nullable<Varchar>,
        code -> Nullable<Int4>,
        email -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    quranAyah (id) {
        id -> Int4,
    }
}

diesel::table! {
    quranLetter (id) {
        id -> Int4,
    }
}

diesel::table! {
    quranSurah (id) {
        id -> Int4,
        name -> Varchar,
        period -> Varchar,
    }
}

diesel::table! {
    quranSurahTag (id) {
        id -> Int4,
        surahID -> Int4,
        name -> Varchar,
        value -> Varchar,
    }
}

diesel::table! {
    quranWord (id) {
        id -> Int4,
    }
}

diesel::table! {
    quran_text (id) {
        id -> Int4,
        surah -> Int4,
        verse -> Int4,
        text -> Text,
    }
}

diesel::table! {
    tanzil_quran (id) {
        id -> Int4,
        sura -> Nullable<Int4>,
        aya -> Nullable<Int4>,
        text -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Text,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    app_logs,
    app_tokens,
    app_users,
    app_verify_codes,
    quranAyah,
    quranLetter,
    quranSurah,
    quranSurahTag,
    quranWord,
    quran_text,
    tanzil_quran,
    users,
);
