import { cors } from "../mod.ts";
import { assertEquals } from "./mod.ts";

Deno.test("Cors Middleware", () => {
    const exampleRequest = new Request("http://localhost", { method: "GET" });
    const exampleResponse = new Response();

    cors(exampleRequest, exampleResponse);

    assertEquals(exampleResponse.headers.get("Access-Control-Allow-Origin"), "*")
    assertEquals(exampleResponse.headers.get("Access-Control-Allow-Credentials"), "true")
});