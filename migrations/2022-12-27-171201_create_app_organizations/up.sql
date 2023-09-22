-- Your SQL goes here

CREATE TABLE app_organizations(
    id serial NOT NULL,
    account_id serial REFERENCES app_accounts(id),
    creator_user_id serial NOT NULL,
    owner_account_id serial,
    profile_image TEXT,
    established_date DATE NOT NULL,
    national_id VARCHAR(11) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT org_fk_user_id_rel FOREIGN KEY(creator_user_id) REFERENCES app_users(id),
    CONSTRAINT app_organizations_id PRIMARY KEY (id),
    UNIQUE(account_id)
);
