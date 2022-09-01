import { Case, assertEquals, runCases } from "./mod.ts";
import { Router, Log, Logger, unsafe } from 'lib';
import { ConnInfo } from "std@http";

Deno.test("Log Object test", async (t) => {
    let lineNumber = 0;

    // const message: Log = { statusCode: 200, createdAt: new Date().toString(), origin: "Origin", requestMethod: 'GET', router: new Router("/"), sourceIp: "0.0.0.0" };
    const request = Promise.resolve(new Request("http://127.0.0.1", { method: "GET" }))
    const response = Promise.resolve(new Response("Example Response"));
    const connInfo = { remoteAddr: { hostname: "113.45.912" } as Deno.NetAddr } as ConnInfo;
    const router = new Router(new URL((await request).url).pathname);

    const createLog = async (): Promise<[Logger, Log]> => {
        const logger = new Logger("./log-test");
        const log = await logger.createLog(request, response, connInfo)
        await logger.writelogToFile(log);

        return [logger, log]
    }

    const cases: Case[] = [
        {
            name: "Write test",
            fn: async () => {
                const [logger, _log] = await createLog()

                // lineNumber += 1; Not Now. because of \n at the EOF
                // TODO: Find way to remove \n at EOF

                // + 1 is for \n at end of line
                const oneLineSize = (await logger.getLine(lineNumber)).length + 1;
                assertEquals(await logger.getLogFileSize(), oneLineSize);
            }
        },
        {
            name: "Get line",
            fn: async () => {
                const [logger, log] = await createLog()
                lineNumber += 1;

                assertEquals(await logger.getLine(lineNumber), `sourceIP: ${unsafe(connInfo.remoteAddr).hostname} ` +
                    `| method: ${(await request).method} | controller: ${router.getController} | ` +
                    `action: ${router.getAction} | id: ${router.getId}` +
                    ` | origin: ${(await request).headers.get("origin")} | created: ${log.createdAt}`);
            }
        },
        {
            name: "Delete log file",
            fn: async () => {
                const log = new Logger("./log-test");
                await log.deleteFile();
            }
        }
    ];

    await runCases(cases, t);
});