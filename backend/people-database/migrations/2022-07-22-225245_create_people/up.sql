CREATE TABLE people (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    birth DATE NOT NULL,
    death DATE,
    description_en TEXT NOT NULL,
    wikipedia_link VARCHAR NOT NULL,
    thumbnail_link VARCHAR NOT NULL,
    links_to_page INTEGER NOT NULL
)