CREATE TABLE app_emails (
    id serial NOT NULL,
    account_id serial NOT NULL,
    creator_user_id serial,
    email TEXT NOT NULL,
    verified BOOLEAN NOT NULL,
    "primary" BOOLEAN NOT NULL,
    deleted BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT app_emails_id PRIMARY KEY (id),
    CONSTRAINT email_fk_user_id_rel FOREIGN KEY(creator_user_id) REFERENCES app_users(id),
    CONSTRAINT fk_email_account_rel FOREIGN KEY(account_id) REFERENCES app_accounts(id)
);
