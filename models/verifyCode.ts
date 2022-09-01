import { Model, DataTypes } from "denodb";

const { INTEGER, STRING } = DataTypes;

class VerifyCode extends Model {
    static table = "app_verify_codes";

    static timestamps = true;

    static fields = {
        id: { primaryKey: true, autoIncrement: true },
        status: STRING,
        code: INTEGER,
        phoneNumber: STRING,
    };

    static defaults = {};
}

export default VerifyCode;