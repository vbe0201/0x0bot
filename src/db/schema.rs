table! {
    message_states (id, created_at) {
        id -> Int8,
        created_at -> Timestamp,
        channel -> Int8,
        author -> Int8,
        content -> Nullable<Text>,
        attachments -> Nullable<Array<Bytea>>,
        deleted -> Bool,
    }
}
