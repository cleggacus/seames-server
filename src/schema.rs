// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tag"))]
    pub struct Tag;
}

diesel::table! {
    blocks (id) {
        id -> Bpchar,
        document_id -> Bpchar,
        line_number -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    documents (id) {
        id -> Bpchar,
        repository_id -> Bpchar,
        slug -> Text,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    image_blocks (block_id) {
        block_id -> Bpchar,
        url -> Nullable<Text>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    repositories (id) {
        id -> Bpchar,
        user_id -> Bpchar,
        slug -> Text,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Tag;

    text_blocks (block_id) {
        block_id -> Bpchar,
        tag -> Tag,
        content -> Nullable<Text>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Bpchar,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(blocks -> documents (document_id));
diesel::joinable!(documents -> repositories (repository_id));
diesel::joinable!(image_blocks -> blocks (block_id));
diesel::joinable!(repositories -> users (user_id));
diesel::joinable!(text_blocks -> blocks (block_id));

diesel::allow_tables_to_appear_in_same_query!(
    blocks,
    documents,
    image_blocks,
    repositories,
    text_blocks,
    users,
);
