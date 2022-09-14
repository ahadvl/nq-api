import { SchemaValidator } from "lib";
import { runCasesConcurrent, Case, } from "./mod.ts";
import { assertRejects } from "std@test";

Deno.test("Test Schema", async (t) => {
    const schema = new SchemaValidator({
        name: { required: true, maxLength: 10, type: "string" },
        username: { required: true, maxLength: 5, type: "string" },
        test: { required: false, maxLength: 10, type: "string" }
    });

    const cases: Case[] = [
        {
            name: "Type validate",
            fn: () => {
                assertRejects(async () => await schema.validate({ name: "Jafar", username: 123 }));
                assertRejects(async () => await schema.validate({ name: 123, username: "x" }));
                assertRejects(async () => await schema.validate({ name: 123, username: 123 }));
            }
        },
        {
            name: "Required validate",
            fn: () => {
                assertRejects(async () => await schema.validate({ name: "" }));
                assertRejects(async () => await schema.validate({ name: "", test: "123" }));
                assertRejects(async () => await schema.validate({ username: " ", test: "123" }));
                assertRejects(async () => await schema.validate({}));
                assertRejects(async () => await schema.validate({ username: " ", name: "123", unwantedData: "" }), "Param is not defined");
                assertRejects(async () => await schema.validate({ username: " ", name: "123", test: "abc", unwantedData: "" }), "Param is not defined");

            }
        },
        {
            name: "Length validate",
            fn: () => {
                assertRejects(async () => await schema.validate({ name: "", username: "13391391931" }));
                assertRejects(async () => await schema.validate({ name: "1234567891011", username: "13391391931" }));
                assertRejects(async () => await schema.validate({ name: "0", username: "133", test: "10000000000000" }));
            }
        },
    ]

    await runCasesConcurrent(cases, t);
})