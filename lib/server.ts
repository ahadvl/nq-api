import { App, Logger } from "lib";
import { serve } from "std@http";

interface ServerConfig {
	/**
	 * Serve server on this ip address for example
	 * localhost OR 192.164.1.1 OR 127.0.0.1
	 */
	ipAddr: string;

	/**
	 * Port
	 */
	port: number;

	/**
	 * SSL certificate
	 */
	cert: string;


	/**
	 * SSL Key
	 */
	key: string;
}

class Server {
	/**
	 * Server config
	 */
	private config: ServerConfig;

	/**
	 * Application
	 */
	private app: App;

	/**
	 * Log Class
	 */
	private logger: Logger;

	constructor(config: ServerConfig, app: App, logger: Logger) {
		this.config = config;
		this.app = app;
		this.logger = logger;
	}

	/**
	 * Start Server
	 * @version 2 : Simplified
	 */
	public run(): void {
		// TODO: Change this to serveTls on production
		serve(async (req: Request, conn) => {
			const res = this.app.returnResponse(req);

			const log = await this.logger.createLog(Promise.resolve(req), res, conn);
			this.logger.writelogToFile(log);
			this.logger.insertLogToDb(log);

			return res;
		}, { hostname: this.config.ipAddr, port: this.config.port })
	}
}

export { Server };