table! {
    scores (id) {
        id -> Int4,
        user_id -> Varchar,
        clear_time -> Int4,
        created_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

joinable!(scores -> users (user_id));

allow_tables_to_appear_in_same_query!(
    scores,
    users,
);
