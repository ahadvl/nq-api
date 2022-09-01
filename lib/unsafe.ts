/**
 * @param value V
 * @returns value as Any
 */
function unsafe<T>(value: T) {
    // deno-lint-ignore no-explicit-any
    return value as any;
}

export { unsafe }