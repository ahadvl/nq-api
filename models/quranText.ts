import { Model, DATA_TYPES } from "denodb";

const { INTEGER, TEXT } = DATA_TYPES;

class QuranText extends Model {
    static table = "quran_text";

    static timestamps = false;

    static fields = {
        id: { primaryKey: true, autoIncrement: true },
        surah: INTEGER,
        verse: INTEGER,
        text: TEXT,
    };

    static defaults = {};
}

export default QuranText;