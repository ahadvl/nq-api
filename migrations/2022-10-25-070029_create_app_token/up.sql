-- Your SQL goes here

CREATE TABLE app_tokens (
    id serial NOT NULL,
    user_id serial NOT NULL,
    token_hash VARCHAR(64) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT app_tokens_id PRIMARY KEY (id)
);