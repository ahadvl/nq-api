CREATE TABLE translations_text (
    id serial NOT NULL,
    translation_id serial NOT NULL,
    ayah_id serial NOT NULL,
    text TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT translation_text_id PRIMARY KEY (id),
    CONSTRAINT fk_translation FOREIGN KEY(translation_id) REFERENCES translations(id),
    CONSTRAINT fk_ayah FOREIGN KEY(ayah_id) REFERENCES quran_ayahs(id)
);
