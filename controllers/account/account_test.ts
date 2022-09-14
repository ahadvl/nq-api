import {Token} from "./token.ts";

Deno.test("Token", async () => {
    const token = new Token(1);

    await token.generate();

    console.log(token.getToken);
});
