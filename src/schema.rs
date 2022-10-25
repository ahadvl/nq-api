// @generated automatically by Diesel CLI.

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
