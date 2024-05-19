-- Simple version for testing  
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(30) NOT NULL,
    password VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    bio VARCHAR,
    is_private BOOLEAN NOT NULL,
    salt VARCHAR NOT NULL
);
