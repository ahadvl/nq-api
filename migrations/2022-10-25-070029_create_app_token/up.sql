CREATE TABLE app_tokens (
    id serial NOT NULL,
    account_id serial NOT NULL,
    token_hash VARCHAR(64) NOT NULL,
    terminated BOOLEAN NOT NULL DEFAULT false,
    terminated_by_id serial,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT app_tokens_id PRIMARY KEY (id),
    CONSTRAINT fk_tkn_account FOREIGN KEY(account_id) REFERENCES app_accounts(id)
    /* CONSTRAINT fk_terminated_account FOREIGN KEY(terminated_by_id) REFERENCES app_accounts(id) */
);
