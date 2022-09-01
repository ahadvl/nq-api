class CustomError extends Error {
    public status: number;

    constructor(status: number, name: string, message: string) {
        super();
        this.status = status;
        this.message = message;
        this.name = name;
    }
}

export default CustomError;