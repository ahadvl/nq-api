-- Your SQL goes here

CREATE TABLE app_accounts (
    id serial NOT NULL,
    username VARCHAR(30) NOT NULL,
    CONSTRAINT app_accounts_id PRIMARY KEY (id),
    UNIQUE(username)
);