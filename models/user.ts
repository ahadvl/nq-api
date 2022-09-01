import { Model, DataTypes } from "denodb";

const { STRING, INTEGER } = DataTypes;

class User extends Model {
    static table = "app_users";

    static timestamps = true;

    static fields = {
        id: { primaryKey: true, autoIncrement: true },
        username: STRING,
        countryCode: INTEGER,
        phoneNumber: INTEGER,
    };

    static defaults = {};
}

export default User;