-- Your SQL goes here
CREATE TABLE quran_ayahs (
    id serial NOT NULL,
    surah_id serial NOT NULL,
    ayah_number serial NOT NULL,
    sajdeh VARCHAR(20) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT quran_ayahs_id PRIMARY KEY (id),
    CONSTRAINT fk_surah_id FOREIGN KEY(surah_id) REFERENCES quran_surahs(id)
);
