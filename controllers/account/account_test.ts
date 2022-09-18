import { assertEquals, assertRejects } from "std@test";
import Account from "./account.ts";
import { Token } from "./token.ts";
import { Router } from 'lib';

Deno.test("Token", async () => {
    const token = new Token(1);

    await token.generate();

    assertEquals(token.getToken.length, 32);
});

Deno.test("Random Code", async () => {
    const exampleRequest = new Request("http://localhost:8080/");
    const exampleRouter = new Router("/account/");

    const account = new Account(exampleRequest, exampleRouter);
    assertRejects(() => account.verify());

    assertEquals(await (await account.post!()).text(), "Action is not Correct");
});