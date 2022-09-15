import * as mod from "std@crypto";
import { encode } from "std@encoding/hex";
import { Salt } from "lib";

class Token {
    readonly userId: number;

    private token: Uint8Array;

    constructor(userId: number) {
        this.userId = userId;
        this.token = new Uint8Array();
    }

    public async generate(): Promise<this> {
        const salt = new Salt()
            .dateAsString()
            .randomBytes(32);

        const userIdAsBytes = new TextEncoder().encode(this.userId.toString());

        const userIdAndSalt = new Uint32Array([...userIdAsBytes, ...salt.getResult]);

        // At the End Hash the random array with SHA-256
        const hash = new Uint8Array(await mod.crypto.subtle.digest("SHA-256", userIdAndSalt));

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
