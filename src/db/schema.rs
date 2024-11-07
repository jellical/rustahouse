// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
    }
}
