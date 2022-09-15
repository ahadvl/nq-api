import { Salt } from 'lib';
import { assertEquals } from 'std@test';

Deno.test("Salt Test", () => {
    const salt = new Salt();
    salt.dateAsString();
    salt.randomBytes(32);

    assertEquals(salt.getResult.length, 45);
});