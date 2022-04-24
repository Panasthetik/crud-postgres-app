table! {
    profiles_db (id) {
        id -> Int4,
        alias -> Varchar,
        full_name -> Nullable<Varchar>,
        photo -> Text,
        mood -> Int4,
    }
}
