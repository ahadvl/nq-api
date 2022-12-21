-- Your SQL goes here

CREATE TABLE app_emails (
    id serial NOT NULL,
    user_id serial NOT NULL,
    email TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT app_emails_id PRIMARY KEY (id)
);