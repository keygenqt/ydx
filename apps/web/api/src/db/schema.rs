// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(
        diesel::query_builder::QueryId,
        serde::Serialize,
        serde::Deserialize,
        Clone,
        Debug,
        diesel::sql_types::SqlType,
    )]
    #[diesel(postgres_type(name = "sort_cards"))]
    pub struct SortCards;
}

diesel::table! {
    cards (id) {
        id -> Int4,
        set_id -> Int4,
        term -> Text,
        definition -> Text,
        image -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    sets (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        icon -> Nullable<Text>,
        description -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SortCards;

    sets_settings (id) {
        id -> Int4,
        set_id -> Int4,
        known_limit -> Nullable<Int4>,
        session_limit -> Nullable<Int4>,
        sort_by -> Nullable<SortCards>,
    }
}

diesel::table! {
    tokens (id) {
        id -> Int4,
        user_id -> Int4,
        refresh_token -> Text,
        device_uuid -> Text,
        expires_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        icon -> Nullable<Varchar>,
        #[max_length = 255]
        fname -> Nullable<Varchar>,
        #[max_length = 255]
        lname -> Nullable<Varchar>,
        #[max_length = 255]
        pwd_hash -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(cards -> sets (set_id));
diesel::joinable!(sets -> users (user_id));
diesel::joinable!(sets_settings -> sets (set_id));
diesel::joinable!(tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(cards, sets, sets_settings, tokens, users,);
