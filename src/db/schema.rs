// @generated automatically by Diesel CLI.

diesel::table! {
    connection_hops (source_connection_id, target_connection_id, hop_order) {
        source_connection_id -> Integer,
        target_connection_id -> Integer,
        hop_order -> Integer,
    }
}

diesel::table! {
    connection_tags (connection_id, tag_id) {
        connection_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    connections (id) {
        id -> Nullable<Integer>,
        name -> Text,
        username -> Text,
        hostname -> Text,
        port -> Nullable<Integer>,
        identity_file -> Nullable<Text>,
        note -> Nullable<Text>,
        created_at -> Integer,
        modified_at -> Nullable<Integer>,
        is_favorite -> Bool,
    }
}

diesel::table! {
    history (id) {
        id -> Nullable<Integer>,
        connection_id -> Integer,
        started_at -> Integer,
        ended_at -> Integer,
        exit_code -> Text,
    }
}

diesel::table! {
    tags (id) {
        id -> Nullable<Integer>,
        name -> Text,
        color -> Text,
    }
}

diesel::joinable!(connection_tags -> connections (connection_id));
diesel::joinable!(connection_tags -> tags (tag_id));
diesel::joinable!(history -> connections (connection_id));

diesel::allow_tables_to_appear_in_same_query!(
    connection_hops,
    connection_tags,
    connections,
    history,
    tags,
);
