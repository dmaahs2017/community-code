table! {
    challenges (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        owner_id -> Nullable<Int4>,
    }
}

table! {
    files (id) {
        id -> Int4,
        real_os_path -> Varchar,
        submission_path -> Varchar,
    }
}

table! {
    submissions (id) {
        id -> Int4,
        challenge_id -> Int4,
        user_id -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(challenges -> users (owner_id));
joinable!(submissions -> challenges (challenge_id));
joinable!(submissions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    challenges,
    files,
    submissions,
    users,
);
