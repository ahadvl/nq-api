CREATE TABLE app_organization_names (
    id serial NOT NULL,
    account_id serial NOT NULL,
    name VARCHAR(300) NOT NULL,
    language VARCHAR(4) NOT NULL,
    CONSTRAINT fk_org_names_account_id FOREIGN KEY(account_id) REFERENCES app_accounts(id),
    CONSTRAINT org_names_id PRIMARY KEY (id)
);
