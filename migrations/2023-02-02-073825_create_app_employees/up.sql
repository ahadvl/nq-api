-- Your SQL goes here
CREATE TABLE app_employees (
    id serial NOT NULL,
    org_account_id serial NOT NULL,
    creator_user_id serial NOT NULL,
    employee_account_id  serial NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT app_employees_id PRIMARY KEY (id),
    UNIQUE(org_account_id, employee_account_id),
    CONSTRAINT employees_fk_user_id_rel FOREIGN KEY(creator_user_id) REFERENCES app_users(id),
    CONSTRAINT fk_account FOREIGN KEY(org_account_id) REFERENCES app_accounts(id),
    CONSTRAINT fk_employee_account FOREIGN KEY(employee_account_id) REFERENCES app_accounts(id)
);
