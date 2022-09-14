class Salt {
    private result: Uint32Array;
    private usedSize: number;

    constructor(size: number) {
        this.result = new Uint32Array(size);
        this.usedSize = 0;
    }

    public dateAsString(): this {
        const dateAsString = Date.now().toString();

        this.string(dateAsString);

        return this;
    }

    public randomBytes(): this {
        const size = this.result.length - this.usedSize;

        for (let i = 0; i < size; i++) {
            const random = Math.floor(Math.random() * 0xFF);
            this.pushToResult(random);
        }

        return this;
    }

    public number(number: number): this {
        this.pushToResult(number);

        return this;
    }

    public string(string: string): this {
        new TextEncoder().encode(string).forEach(byte => this.pushToResult(byte));

        return this;
    }

    private pushToResult(value: number) {
        this.result.set([value], this.usedSize);
        this.usedSize += 1;
    }

    get getResult() {
        return this.result;
    }
}

export default Salt;
