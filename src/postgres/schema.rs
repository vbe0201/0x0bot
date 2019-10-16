table! {
    messages (id) {
        id -> BigUint,
        channel -> BigUint,
        author -> BigUint,
        content -> Text,
        created_at -> Timestamp,
    }
}

table! {
    message_edits (id) {
        id -> Int4,
        message_id -> BigUint,
        content -> Text,
        edited_at -> Timestamp,
    }
}

joinable!(messages -> message_edits (message_id));

allow_tables_to_appear_in_same_query!(messages, message_edits);
