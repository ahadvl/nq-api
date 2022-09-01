import { assertEquals, assertRejects, assert } from "std@test";

interface Case {
    name: string;
    fn: () => void;
}

async function runCasesConcurrent(cases: Case[], t: Deno.TestContext): Promise<void> {
    await Promise.all(cases.map(c => {
        t.step({
            name: c.name,
            fn: c.fn,
            sanitizeOps: false,
            sanitizeResources: false,
            sanitizeExit: false,
        })
    }))
}

async function runCases(cases: Case[], t: Deno.TestContext): Promise<void> {
    for (const c of cases) {
        await t.step(c);
    }
}

export type { Case }
export { assertEquals, assertRejects, runCasesConcurrent, runCases, assert }