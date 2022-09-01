import { Database, Model } from "denodb";

class Db {
    private database: Database;
    public models: typeof Model[];

    constructor(database: Database) {
        this.database = database;
        this.models = [];
    }

    /**
     * Add a new model(tabel)
     * @param m Model 
     * @returns this
     */
    public pushModel(m: typeof Model): this {
        this.models.push(m);

        return this;
    }

    /**
     * Execute the final functions: Add the models and sync the database
     * @returns this
     */
    public done(): this {
        this.database.link(this.models);
        (async () => await this.database.sync())();

        return this;
    }
}

export { Db }