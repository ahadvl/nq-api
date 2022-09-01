class Router {
    private path: string;
    private conrtoller: string | null;
    private action: string | null;
    private id: string | null;

    constructor(path: string) {
        this.path = path;
        this.conrtoller = null;
        this.action = null;
        this.id = null;
    }

    public parse(): this {
        const splited = this.path.split("/").filter(item => item !== "");

        if (splited[0]) this.conrtoller = splited[0];
        if (splited.length <= 2) this.id = splited[1];
        else {
            this.action = splited[1]
            this.id = splited[2]
        }

        return this;
    }

    get getController() {
        return this.conrtoller
    }

    get getAction() {
        return this.action
    }

    get getId() {
        return this.id
    }
}

export { Router };