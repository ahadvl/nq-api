-- Your SQL goes here

CREATE TABLE quran_words(
    id serial NOT NULL,
    ayah_id serial NOT NULL,
    word TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT quran_words_id PRIMARY KEY (id),
    CONSTRAINT fk_w_ayah FOREIGN KEY(ayah_id) REFERENCES quran_ayahs(id)
);
