-- Your SQL goes here

CREATE TABLE quran_text (
    id serial NOT NULL,
    surah_id int4 NOT NULL DEFAULT 0,
   	verse_number int4 NOT NULL DEFAULT 0,
    "text" text NOT NULL,
    CONSTRAINT quran_text_pkey PRIMARY KEY (id)
);