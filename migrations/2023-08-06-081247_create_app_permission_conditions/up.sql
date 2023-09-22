CREATE TABLE app_permission_conditions (
    id serial NOT NULL,
    uuid uuid DEFAULT uuid_generate_v4 () NOT NULL,
    creator_user_id serial NOT NULL,
    permission_id serial NOT NULL,
    name VARCHAR(450) NOT NULL,
    value VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT app_permission_conditions_id PRIMARY KEY (id),
    CONSTRAINT permission_condition_fk_user_id_rel FOREIGN KEY(creator_user_id) REFERENCES app_users(id),
    CONSTRAINT fk_cond_perm_id FOREIGN KEY(permission_id) REFERENCES app_permissions(id)
        on delete cascade
);
