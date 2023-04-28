-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE IF NOT EXISTS users(
    id uuid NOT NULL UNIQUE PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(256) NOT NULL UNIQUE,
    username VARCHAR(256) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    logged_at TIMESTAMP WITH TIME ZONE NOT NULL  DEFAULT NOW()
); 

CREATE TABLE IF NOT EXISTS friends(
    id serial NOT NULL UNIQUE PRIMARY KEY,
    user_id uuid NOT NULL REFERENCES users(id),
    friend VARCHAR(256) NOT NULL
);