-- Your SQL goes here

CREATE TABLE app_tokens (
    id serial NOT NULL,
    user_id serial NOT NULL,
    token_hash VARCHAR(64) NOT NULL,
    terminated BOOLEAN NOT NULL DEFAULT false,
    terminated_by_id serial,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT app_tokens_id PRIMARY KEY (id)
);