-- Your SQL goes here

CREATE TABLE app_organizations_table(
    id serial NOT NULL,
    username VARCHAR(30) NOT NULL,
    "name" VARCHAR(200) NOT NULL,
    profile_image TEXT,
    established_date DATE NOT NULL,
    national_id VARCHAR(11) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT app_organizations PRIMARY KEY (id),
    UNIQUE(username)
);
