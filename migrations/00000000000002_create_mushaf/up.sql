CREATE TABLE mushafs (
    id serial NOT NULL,
    name VARCHAR(200),
    source VARCHAR(300),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT mushaf_id PRIMARY KEY (id)
);