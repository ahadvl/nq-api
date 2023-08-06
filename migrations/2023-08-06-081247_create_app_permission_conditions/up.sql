CREATE TABLE app_permission_conditions (
    id serial NOT NULL,
    permission_id serial NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT app_permission_conditions_id PRIMARY KEY (id),
    UNIQUE(permission_id),
    CONSTRAINT fk_cond_perm_id FOREIGN KEY(permission_id) REFERENCES app_permissions(id)
);
