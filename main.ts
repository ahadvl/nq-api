import { config } from "std@env";
import { Db, Server, cors, Logger, App } from 'lib';
import { PostgresConnector, Database } from "denodb";
import { LogModel, VerifyCode, User } from "models";
import { Status, Account } from "./controllers/mod.ts";

const env = await config();

const connector = new PostgresConnector({
    database: env.DATABASE || "postgres",
    host: env.DATABASE_HOST || "localhost",
    username: env.DATABASE_USERNAME || "postgres",
    password: env.DATABASE_PASSWORD || "postgres"
});

new Db(new Database(connector))
    .pushModel(LogModel)
    .pushModel(VerifyCode)
    .pushModel(User)
    .done();

const logger = new Logger(`logs/log.log`);

const app = new App();
app.pushController(Status, "status");
app.pushController(Account, "account");

app
    .useMiddleware(cors)

new Server({ ipAddr: "127.0.0.1", port: 8080, cert: "cert.pem", key: "private.pem" }, app, logger)
    .run();
