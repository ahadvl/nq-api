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

/**
 * Runs (repeat) times fn
 * And return just one result
 * And result every time overwrites
 */
const times = <T>(fn: () => T, repeat: number) => {
    let result;
    for (let i = 0; i < repeat; i++) {
        result = fn();
    }

    return result;
}

class TokenGenerator {
    readonly value: Uint32Array;
    readonly salt: number;
    readonly round: number;

    private token: Uint8Array;

    constructor(value: Uint32Array, salt: number, round = 1) {
        this.value = value;
        this.salt = salt
        this.round = round;
        this.token = new Uint8Array();
    }

    public async generate(): Promise<this> {
        // Map the value bytes and for each run this op and reverse result array
        // And Run this Function 0..this.round
        const result = times(
            () => this.value.map(byte => this.salt * (byte * this.round) * (byte ** 2)).reverse(),
            this.round
        );

        // At the End Hash the random array with SHA-256
        const hash = new Uint8Array(await mod.crypto.subtle.digest("SHA-256", result!));

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
