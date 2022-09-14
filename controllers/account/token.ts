import * as mod from "std@crypto";
import { encode } from "std@encoding/hex";
import { Salt } from "lib";

class Token {
    readonly input: number;

    private token: Uint8Array;

    constructor(input: number) {
        this.input = input;
        this.token = new Uint8Array();
    }

    public async generate(): Promise<this> {
        const salt = new Salt(32)
            .dateAsString()
            .string(this.input.toString())
            .randomBytes();

        // At the End Hash the random array with SHA-256
        const hash = new Uint8Array(await mod.crypto.subtle.digest("SHA-256", salt.getResult));

        this.token = hash;

        return this;
    }

    get getToken() {
        return this.token;
    }

    get getTokenAsString() {
        return new TextDecoder().decode(encode(this.token));
    }
}

export { Token }
