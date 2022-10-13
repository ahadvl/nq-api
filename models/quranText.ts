import { Model, DATA_TYPES } from "denodb";

const { INTEGER, TEXT } = DATA_TYPES;

class QuranText extends Model {
    static table = "quran_text";

    static timestamps = false;

    static fields = {
        index: { primaryKey: true, autoIncrement: true },
        sura: INTEGER,
        aya: INTEGER,
        text: TEXT,
    };

    static defaults = {};
}

export default QuranText;