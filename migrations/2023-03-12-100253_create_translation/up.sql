CREATE TABLE translations (
    id serial NOT NULL,
    translator_id serial NOT NULL,
    language VARCHAR(5) NOT NULL,
    release_year DATE,
    source VARCHAR(100),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT translation_id PRIMARY KEY (id)
);
