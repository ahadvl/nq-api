import { Router, Db, HttpMethod } from 'lib';
import { ConnInfo } from "std@http";
import { LogModel } from "models";

export interface Log {
	/**
	 * Status of log as Http Status Code
	 */
	statusCode: number;

	/**
	 * Source(Client) Ip
	 */
	sourceIp: string;

	/**
	 * Request Method
	 */
	requestMethod: HttpMethod;

	/**
	 * Request parsed Router
	 */
	router: Router;

	/**
	 * Request Origin
	 */
	origin: string | null;

	/**
	 * Creation Date
	 */
	createdAt: Date | string;
}

/**
 * Creates a new Date object and returns fomrated date:? YYYY-MM-DD-HH-Min-Sec as String
 * @version beta
 * @returns {string} Current date
 */
const getDateAsString = (date: Date): string =>
	`${date.getFullYear()}-${date.getMonth()}-${date.getDay()}_${date.getHours()}:${date.getMinutes()}:${date.getSeconds()}`;

/**
 * @param log log object
 * @returns Formated log
 */
const logToString = (log: Log): string =>
	`sourceIP: ${log.sourceIp} ` +
	`| method: ${log.requestMethod} | controller: ${log.router.getController} | ` +
	`action: ${log.router.getAction} | id: ${log.router.getId}` +
	` | origin: ${log.origin} | created: ${log.createdAt}\n`

class Logger {
	/**
	 * File address to save a log file
	 */
	private file_addr: string;

	/**
	 * Database Object
	 */
	private db: Db | null;

	constructor(logFileAddr: string, database: Db | null = null) {
		this.file_addr = logFileAddr;
		this.db = database;
	}

	public async writelogToFile(log: Log): Promise<void> {
		// Open file and write log
		await Deno.writeTextFile(this.file_addr, logToString(log), { create: true, append: true })
	}

	/**
	 * Ask for line content
	 * @param index Line number
	 * @returns {Promise<string>}
	 */
	public async getLine(index: number): Promise<string> {
		const file = await Deno.readTextFile(this.file_addr);
		const lines = file.split("\n");

		return lines[index];
	}

	/**
	 * Removes Log file
	 */
	public async deleteFile(): Promise<void> {
		await Deno.remove(this.file_addr);
	}

	/**
	 * Gatters data from request and response and Connection.
	 * Create log And push 
	 * @param req 
	 * @param res 
	 * @param conn 
	 */
	public async createLog(
		req: Promise<Request>,
		res: Promise<Response>,
		conn: ConnInfo,
	): Promise<Log> {
		// Copy of request and response
		const request = (await req).clone();
		const response = (await res).clone();

		// Get remote addr (client Ip)
		const sourceIp = (conn.remoteAddr as Deno.NetAddr).hostname;
		const router = new Router(new URL(request.url).pathname);

		const origin = request.headers.get("origin");

		return {
			statusCode: response.status,
			origin: origin,
			sourceIp: sourceIp,
			router: router,
			createdAt: getDateAsString(new Date()),
			requestMethod: request.method as HttpMethod,
		} as Log;
	}

	/**
	 * Logs Request and Connection to Database
	 */
	public async insertLogToDb(log: Log): Promise<void> {
		await LogModel.create({
			status: log.statusCode,

			source_ip: log.sourceIp,

			method: log.requestMethod,

			controller: log.router.getController,
			action: log.router.getAction,
			requested_id: log.router.getId,

			origin: log.origin,
		});
	}

	/**
	 * Returns log file size
	 */
	public async getLogFileSize() {
		const stat = await Deno.stat(this.file_addr);

		return stat.size;
	}
}

export { Logger };
