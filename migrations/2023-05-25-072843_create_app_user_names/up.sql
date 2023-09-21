CREATE TABLE app_user_names(
    id serial NOT NULL,
    account_id serial NOT NULL,
    creator_user_id serial NOT NULL,
    primary_name boolean NOT NULL,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(200) NOT NULL,
    language VARCHAR(4) NOT NULL,
    CONSTRAINT fk_user_names_account_id FOREIGN KEY(account_id) REFERENCES app_accounts(id),
    CONSTRAINT user_name_fk_user_id_rel FOREIGN KEY(creator_user_id) REFERENCES app_users(id),
    CONSTRAINT user_names_id PRIMARY KEY (id)
);