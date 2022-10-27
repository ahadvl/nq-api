-- Your SQL goes here

CREATE TABLE app_users (
    id serial NOT NULL,
    username VARCHAR(30) NOT NULL,
    email TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT app_users_id PRIMARY KEY (id)
);