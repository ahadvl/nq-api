CREATE TABLE app_users (
    id serial NOT NULL,
    uuid uuid DEFAULT uuid_generate_v4 () NOT NULL,
    account_id serial NOT NULL,
    birthday DATE,
    profile_image TEXT,
    language VARCHAR(4) DEFAULT 'en',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT app_users_id PRIMARY KEY (id),
    UNIQUE(account_id),
    CONSTRAINT fk_user_account_rel FOREIGN KEY(account_id) REFERENCES app_accounts(id)
);
