class Salt {
    private result: number[];

    constructor() {
        this.result = [];
    }

    public dateAsString(): this {
        const dateAsString = Date.now().toString();

        this.stringToBytes(dateAsString);

        return this;
    }

    public randomBytes(size: number): this {
        for (let i = 0; i < size; i++) {
            const random = Math.floor(Math.random() * 0xFF);
            this.result.push(random);
        }

        return this;
    }

    private stringToBytes(string: string): this {
        new TextEncoder().encode(string).forEach(byte => this.result.push(byte));

        return this;
    }

    get getResult() {
        return this.result;
    }
}

export default Salt;
