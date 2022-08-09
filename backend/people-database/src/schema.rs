table! {
    people (id) {
        id -> Int4,
        name -> Varchar,
        birth -> Date,
        death -> Nullable<Date>,
        description_en -> Text,
        wikipedia_link -> Varchar,
        thumbnail_link -> Varchar,
        links_to_page -> Int4,
    }
}

table! {
    users (userid) {
        userid -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        pw -> Text,
        birthday -> Date,
        admin_role -> Bool,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    people,
    users,
);
