CREATE TABLE users (
    userid SERIAL NOT NULL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT NOT NULL,
    pw TEXT NOT NULL,
    birthday DATE NOT NULL,
    admin_role BOOLEAN NOT NULL,
    created_at TIMESTAMP NOT NULL,
    UNIQUE(email)
);