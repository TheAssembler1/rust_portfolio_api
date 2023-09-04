// @generated automatically by Diesel CLI.

diesel::table! {
    blogs (id) {
        id -> Integer,
        created_at -> Datetime,
        updated_at -> Nullable<Datetime>,
        #[max_length = 256]
        title -> Varchar,
        summary -> Text,
        body -> Mediumtext,
        visible -> Bool,
    }
}
