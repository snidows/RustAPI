-- Your SQL goes here
CREATE TABLE users(
    id serial PRIMARY KEY,
    name VARCHAR NOT NULL,
    city VARCHAR NOT NULL,
    cep VARCHAR NOT NULL,
    tel VARCHAR NOT NULL
)