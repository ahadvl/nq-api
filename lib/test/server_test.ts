import { Case, assertEquals, runCasesConcurrent } from "./mod.ts";
import { Router } from "../mod.ts";

Deno.test("Parse Router Path", async (t) => {
    const cases: Case[] = [
        {
            name: "/controller/action/id",
            fn: () => {
                const parsed = new Router(cases[0].name).parse();
                assertEquals(parsed.getController, "controller")
                assertEquals(parsed.getAction, "action")
                assertEquals(parsed.getId, "id")
            }
        },
        {
            name: "/controller/id",
            fn: () => {
                const parsed = new Router(cases[1].name).parse();
                assertEquals(parsed.getController, "controller")
                assertEquals(parsed.getId, "id")
            }
        },
        {
            name: "/controller",
            fn: () => {
                const parsed = new Router(cases[2].name).parse();
                assertEquals(parsed.getController, "controller")
            }
        }

    ];

    await runCasesConcurrent(cases, t);
})
