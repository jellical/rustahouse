-- Your SQL goes here
CREATE TABLE users
(
    id         UUID not null
        constraint users_id_pk
            primary key,
    email      TEXT not null
        constraint users_email_pk
            unique,
    first_name TEXT,
    last_name  TEXT
)