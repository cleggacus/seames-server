// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Bpchar,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
