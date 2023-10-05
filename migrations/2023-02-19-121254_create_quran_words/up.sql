CREATE TABLE quran_words(
    id serial NOT NULL,
    uuid uuid DEFAULT uuid_generate_v4 () NOT NULL,
    creator_user_id serial NOT NULL,
    ayah_id serial NOT NULL,
    word TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT quran_words_id PRIMARY KEY (id),
    CONSTRAINT word_fk_user_id_rel FOREIGN KEY(creator_user_id) REFERENCES app_users(id),
    CONSTRAINT fk_w_ayah FOREIGN KEY(ayah_id) REFERENCES quran_ayahs(id)
        on delete cascade
);
