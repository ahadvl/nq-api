CREATE TABLE app_tokens (
    id serial NOT NULL,
    account_id serial NOT NULL,
    creator_user_id serial,
    token_hash VARCHAR(64) NOT NULL,
    terminated BOOLEAN NOT NULL DEFAULT false,
    terminated_by_id serial,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT app_tokens_id PRIMARY KEY (id),
    CONSTRAINT fk_tkn_account FOREIGN KEY(account_id) REFERENCES app_accounts(id),
    CONSTRAINT token_fk_user_id_rel FOREIGN KEY(creator_user_id) REFERENCES app_users(id)
);
