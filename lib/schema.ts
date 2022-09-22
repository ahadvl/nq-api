import { CustomError } from "lib";

interface Schema {
    /**
     * Param Type
     * x: ""
     * 
     * typeof x.value
     */
    type: string;

    /**
     * Is param Required ?
     */
    required: boolean;

    /**
     * Max Length of value
     */
    maxLength: number;

    /**
     * Min length of value
     */
    minLength: number;
}

// Schema is for client request verifying 
// For example check if request body is equal to <Foo> Type
class SchemaValidator {
    schemas: Record<string, Schema>;

    constructor(params: Record<string, Schema>) {
        this.schemas = params;
    }

    /**
     * checks if key is required or not
     * @param key Key of Target
     */
    private isRequired(key: string): boolean {
        return Boolean(this.schemas[key]?.required);
    }

    /**
     * checks if key exists
     * @param key Key of 
     */
    private checkNoUnwantedData(key: string): Promise<boolean> {
        if (!this.schemas[key]) return Promise.reject(new CustomError(400, 'Schema', `${key} is Unwanted data!`));
        return Promise.resolve(true);
    }

    /**
     * Check required option
     * @param thisKey Param key
     * @param targetKeys target Keys
     */
    private checkRequirement(thisKey: string, targetKeys: string[]): Promise<boolean> {
        if (targetKeys.includes(thisKey) === false) {
            if (this.isRequired(thisKey)) {
                return Promise.reject(new CustomError(400, 'Schema', `Cant find ${thisKey} but its required`));
            }
        }
        return Promise.resolve(true);
    }

    /**
     * Checks if types of target is valid
     * @param thisKey 
     * @param targetValue 
     */
    private checkType<T>(thisKey: string, targetValue: T): Promise<boolean> {
        const paramType = this.schemas[thisKey].type;
        const targetType = typeof targetValue;

        if (paramType !== targetType)
            return Promise.reject(new CustomError(400, 'Schema', `Param Type is not correct, expected ${paramType} found ${targetType}`));

        return Promise.resolve(true);
    }

    /**
     * Checks if length of target is valid
     * @param thisKey 
     * @param targetValue 
     */
    private lengthCheck(thisKey: string, targetValue: any): Promise<boolean> {
        const requiredMaxLength = this.schemas[thisKey].maxLength;
        const requiredMinLength = this.schemas[thisKey].minLength;
        const targetLength = targetValue.toString().length;

        if (targetLength > requiredMaxLength)
            return Promise.reject(new CustomError(400, 'Schema', `Target length is more than expected length ${requiredMaxLength} found ${targetLength}`));
        else if (targetLength < requiredMinLength)
            return Promise.reject(new CustomError(400, 'Schema', `Target length is less than expected length ${requiredMinLength} found ${targetLength}`));

        return Promise.resolve(true);
    }

    /**
     * Check if target is correct
     * @param target Target to check
     * @returns 
     */
    public async validate(target: Record<string, any>): Promise<boolean> {
        const schemasKeys = Object.keys(this.schemas);
        const targetKeys = Object.keys(target);

        for (const key of targetKeys) {
            await this.checkNoUnwantedData(key)
        }

        for (const key of schemasKeys) {
            await this.checkRequirement(key, targetKeys);
            await this.checkType(key, target[key])
            await this.lengthCheck(key, target[key])
        }

        return true;
    }
}

export { SchemaValidator };