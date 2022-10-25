-- Your SQL goes here

CREATE TABLE app_verify_codes (
    id serial NOT NULL,
    code INT NOT NULL,
    email TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT verify_codes_id PRIMARY KEY (id)
);