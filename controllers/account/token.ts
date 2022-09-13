import * as mod from "std@crypto";
import { encode } from "std@encoding/hex";

/**
 * Recives String and returns Array of numbers
 * Covert each char of string to number as UTF-16
 */
const stringToBytes = (input: string) => {
    const array = [];

    for (let index = 0; index < input.length; index++) {
        array.push(input.charCodeAt(index));
    }

    return Uint32Array.from(array);
}

class TokenGenerator {
    readonly input: Uint8Array;

    private token: Uint8Array;

    constructor(input: Uint8Array) {
        this.input = input;
        this.token = new Uint8Array();
    }

    public async generate(): Promise<this> {
        // At the End Hash the random array with SHA-256
        const hash = new Uint8Array(await mod.crypto.subtle.digest("SHA-256", this.input));

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

export { stringToBytes, TokenGenerator }
