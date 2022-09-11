import * as mod from "std@crypto";
import { encode } from "std@encoding/hex";

const stringToBytes = (input: string) => {
    const array = [];

    for (let index = 0; index < input.length; index++) {
        array.push(input.charCodeAt(index));
    }

    return Uint32Array.from(array);
}

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

    /**
     * generate
     */
    public async generate(): Promise<this> {
        const result = times(
            () => this.value.map(byte => this.salt * (byte * this.round) * (byte ** 2)).reverse(),
            this.round
        );

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