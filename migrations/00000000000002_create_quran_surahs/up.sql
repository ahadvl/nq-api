CREATE TABLE quran_surahs (
    id serial NOT NULL,
    name VARCHAR(50) NOT NULL, 
    period VARCHAR(50), 
    number serial NOT NULL,
    bismillah_status VARCHAR(10) NOT NULL,
    bismillah_text TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT quran_surahs_id PRIMARY KEY (id)
);
