table! {
    users (id) {
        id -> Int4,
        uuid -> Text,
        name -> Text,
        active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
