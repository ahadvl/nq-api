import { Model, DATA_TYPES } from "denodb";

const { STRING } = DATA_TYPES;

class QuranSurah extends Model {
    static table = "quranSurah";

    static timestamps = false;

    static fields = {
        id: { primaryKey: true, autoIncrement: true },
        name: STRING,
        period: STRING,
    };

    static defaults = {};
}

export default QuranSurah;