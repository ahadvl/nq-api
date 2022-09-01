import { ResponseFunction, HttpMethod, Router } from 'lib';

export interface BaseController {
    request: Request;
    router: Router;

    get?: ResponseFunction;
    post?: ResponseFunction;
    put?: ResponseFunction;
    delete?: ResponseFunction;
    options?: ResponseFunction;
    head?: ResponseFunction;
    trace?: ResponseFunction;
    patch?: ResponseFunction;
}

abstract class Controller implements BaseController {
    request: Request;
    router: Router;

    constructor(request: Request, router: Router) {
        this.request = request;
        this.router = router;

        // Give This to the methods
        // If We dont do that method will get this as undefined
        this.get = this.get?.bind(this);
        this.post = this.post?.bind(this);
        this.put = this.put?.bind(this);
        this.delete = this.delete?.bind(this);
        this.head = this.head?.bind(this);
        this.options = this.options?.bind(this);
        this.patch = this.patch?.bind(this);
        this.trace = this.trace?.bind(this);
    }

    get?(): Promise<Response>;
    put?(): Promise<Response>;
    post?(): Promise<Response>;
    delete?(): Promise<Response>;
    options?(): Promise<Response>;
    head?(): Promise<Response>;
    trace?(): Promise<Response>;
    patch?(): Promise<Response>;

    /**
     * Gets Method returns Handle function
     * @param method HttpMethod
     * @returns 
     */
    public getHandleFromMethod(method: HttpMethod): ResponseFunction | undefined {
        let result: ResponseFunction | undefined = undefined;

        switch (method) {
            case "GET":
                result = this.get
                break;

            case "POST":
                result = this.post
                break;

            case "PUT":
                result = this.put
                break;

            case "DELETE":
                result = this.delete
                break;

            case "HEAD":
                result = this.head
                break;

            case "TRACE":
                result = this.trace
                break;

            case "PATCH":
                result = this.patch
                break;

            default:
                break
        }

        return result;
    }
}

export { Controller }