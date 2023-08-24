CREATE TABLE app_accounts (
    id serial NOT NULL,
    uuid uuid DEFAULT uuid_generate_v4 () NOT NULL,
    username VARCHAR(30) NOT NULL,
    account_type TEXT NOT NULL,
    CONSTRAINT app_accounts_id PRIMARY KEY (id),
    UNIQUE(username)
);
