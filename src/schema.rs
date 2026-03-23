// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 255]
        image_url -> Nullable<Varchar>,
        price -> Numeric,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
