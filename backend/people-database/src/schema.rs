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
