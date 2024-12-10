// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        nickname -> Text,
        full_name -> Text,
        email -> Text,
        phone_num -> Text,
        activated_at -> Nullable<Timestamp>,
        ts -> Timestamp,
    }
}
