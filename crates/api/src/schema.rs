// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        contact_number -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 255]
        hash_password -> Varchar,
        email_verified -> Nullable<Bool>,
    }
}
