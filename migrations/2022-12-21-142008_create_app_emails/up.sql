-- Your SQL goes here

CREATE TABLE app_emails (
    id serial NOT NULL,
    account_id serial NOT NULL,
    email TEXT NOT NULL,
    verified BOOLEAN NOT NULL,
    "primary" BOOLEAN NOT NULL,
    deleted BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT app_emails_id PRIMARY KEY (id),
    CONSTRAINT fk_account FOREIGN KEY(account_id) REFERENCES app_accounts(id)
);
