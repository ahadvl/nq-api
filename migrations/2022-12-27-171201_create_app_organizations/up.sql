-- Your SQL goes here

CREATE TABLE app_organizations (
    id serial NOT NULL,
    name VARCHAR(50) NOT NULL,
    profile_image TEXT,
    established_date TIMESTAMPTZ NOT NULL,
    national_id serial NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT app_organizations PRIMARY KEY (id)
);
