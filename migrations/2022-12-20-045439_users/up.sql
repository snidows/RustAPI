-- Your SQL goes here
CREATE TABLE users(
    id serial PRIMARY KEY,
    uuid VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    street VARCHAR NOT NULL,
    city VARCHAR NOT NULL,
    state VARCHAR NOT NULL
)