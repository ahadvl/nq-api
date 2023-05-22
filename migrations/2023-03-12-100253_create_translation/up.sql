CREATE TABLE translations (
    id serial NOT NULL,
    uuid uuid DEFAULT uuid_generate_v4 (),
    translator_id serial NOT NULL,
    language VARCHAR(5) NOT NULL,
    release_year DATE,
    source VARCHAR(300),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT translation_id PRIMARY KEY (id)
);
