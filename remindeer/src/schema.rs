// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
        last_modified -> Timestamp,
        #[max_length = 255]
        phone_number -> Varchar,
        unid -> Uuid,
        #[max_length = 100]
        photo -> Varchar,
    }
}
