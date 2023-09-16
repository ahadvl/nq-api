CREATE TABLE quran_surahs (
    id serial NOT NULL,
    uuid uuid DEFAULT uuid_generate_v4 () NOT NULL,
    name VARCHAR(50) NOT NULL,
    period VARCHAR(50),
    number serial NOT NULL,
    bismillah_status VARCHAR(10) NOT NULL,
    mushaf_id serial NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT quran_surahs_id PRIMARY KEY (id),
    CONSTRAINT fk_mushaf_id FOREIGN KEY(mushaf_id) REFERENCES mushafs(id)
);
