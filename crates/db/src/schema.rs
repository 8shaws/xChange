// @generated automatically by Diesel CLI.

diesel::table! {
    kyc (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        document_type -> Varchar,
        #[max_length = 255]
        document_number -> Varchar,
        #[max_length = 255]
        issue_country -> Varchar,
        expiry_date -> Timestamp,
        #[max_length = 1000]
        document_front_url -> Varchar,
        #[max_length = 1000]
        document_back_url -> Varchar,
        #[max_length = 1000]
        selfie_url -> Varchar,
        verification_status -> Varchar,
        submitted_at -> Timestamp,
        verified_at -> Nullable<Timestamp>,
        rejected_at -> Nullable<Timestamp>,
        #[max_length = 1000]
        rejection_reason -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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

diesel::joinable!(kyc -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    kyc,
    users,
);
