import { Model, DATA_TYPES } from "denodb";

const { STRING, INTEGER } = DATA_TYPES;

class TokenModel extends Model {
    static table = "app_tokens";

    static timestamps = true;

    static fields = {
        id: { primaryKey: true, autoIncrement: true },
        userId: INTEGER,
        token: STRING,
    };

    static defaults = {};
}

export default TokenModel;