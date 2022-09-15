import { assertEquals } from "std@test";
import { Token } from "./token.ts";

Deno.test("Token", async () => {
    const token = new Token(1);

    await token.generate();

    assertEquals(token.getToken.length, 32);
});
