// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        photo -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        phone_number -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
