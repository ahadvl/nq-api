import { Model, DataTypes } from "denodb";

const { STRING } = DataTypes;

class User extends Model {
    static table = "app_users";

    static timestamps = true;

    static fields = {
        id: { primaryKey: true, autoIncrement: true },
        username: STRING,
        email: STRING,
    };

    static defaults = {};
}

export default User;