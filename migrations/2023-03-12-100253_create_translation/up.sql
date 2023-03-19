CREATE TABLE translations (
    id serial NOT NULL,
    translator_id serial NOT NULL,
    language serial NOT NULL,
    release_year DATE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT translation_id PRIMARY KEY (id)
);
