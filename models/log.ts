import { Model, DataTypes } from "denodb";

const { INTEGER, STRING } = DataTypes;

class LogModel extends Model {
    static table = "app_logs";

    static timestamps = true;

    static fields = {
        id: { primaryKey: true, autoIncrement: true },
        status: INTEGER,
        source_ip: STRING,
        method: STRING,
        controller: STRING,
        action: STRING,
        requested_id: INTEGER,
        origin: STRING,
    };

    static defaults = {};
}

export default LogModel;