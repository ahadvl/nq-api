import { assertEquals } from "std@test";
import { Router, Controller } from 'lib';

Deno.test("Controller Test", () => {
    class Example extends Controller {
        get() {
            return Promise.resolve(new Response("Hello world"))
        }
    }

    const e = new Example(new Request("http://example.com"), new Router("/"));

    assertEquals(e.get(), Promise.resolve(new Response("Hello world")));
});