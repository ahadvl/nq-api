import { Salt } from 'lib';
import { assertThrows, assertEquals, assertNotEquals } from 'std@test';

Deno.test("Salt Test", () => {
    const salt = new Salt(32);

    salt.number(0xFF);
    assertEquals(salt.getResult[0], 0xFF);

    salt.string("Hello");
    assertEquals(salt.getResult[1], 72);
    assertEquals(salt.getResult[2], 101);
    assertNotEquals(salt.getResult[10], 111);

    salt.randomBytes();

    assertThrows(() => salt.number(123));
});