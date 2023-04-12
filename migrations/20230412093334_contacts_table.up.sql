CREATE TABLE IF NOT EXISTS contacts (
    id serial PRIMARY KEY,
    name VARCHAR (255) NOT NULL,
    phone_no BIGINT NOT NULL,
    email VARCHAR (255) NOT NULL
);