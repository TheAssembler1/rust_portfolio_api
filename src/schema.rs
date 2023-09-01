// @generated automatically by Diesel CLI.

diesel::table! {
    blogs (id) {
        id -> Integer,
        #[max_length = 256]
        title -> Varchar,
        body -> Mediumtext,
        visible -> Bool,
    }
}
