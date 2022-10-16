import { Model, DATA_TYPES } from "denodb";

const { STRING } = DATA_TYPES;

class QuranSura extends Model {
    static table = "quranSura";

    static timestamps = false;

    static fields = {
        id: { primaryKey: true, autoIncrement: true },
        name: STRING,
        period: STRING,
    };

    static defaults = {};
}

export default QuranSura;