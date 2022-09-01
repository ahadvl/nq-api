import { Controller, Router, CustomError } from "lib";

export type HttpMethod = "GET" | "POST" | "PUT" | "DELETE" | "OPTIONS" | "HEAD" | "TRACE" | "PATCH";

export type ResponseFunction = () => Promise<Response>;

/**
 * Middleware type
 */
export type Middleware = (req: Request, res: Response, ...args: any[]) => void;


const METHOD_NOT_ALLOWED = () => new Response("Method not Allowed", { status: 403 });
const CONTROLLER_NOT_FOUND = () => new Response("Controller Not found", { status: 404 });
const CONTROLLER_NOT_SPECIFIED = () => new Response("Controller is not specified", { status: 400 })

type ExtendedController = { new(request: Request, router: Router): Controller };

class App {
    controllers: Map<string, ExtendedController>;

    /**
     * All the middlewares
     */
    private readonly middlewares: Middleware[];

    constructor() {
        this.controllers = new Map();
        this.middlewares = [];
    }

    /**
     * Handles a request and returns Response
     * @param request
     */
    public async returnResponse(request: Request): Promise<Response> {
        const { url, method } = request.clone();

        const httpMethod = method as HttpMethod;

        const router = new Router(new URL(url).pathname).parse();

        // If there is no Controller sent in path then return 400 which means Bad Request
        if (router.getController === null) return CONTROLLER_NOT_SPECIFIED();

        const controller = this.controllers.get(router.getController);

        // Check if Controller exists
        if (controller === undefined) return CONTROLLER_NOT_FOUND();

        const controllerObject = new controller(request.clone(), router);

        const handleFromMethod = controllerObject.getHandleFromMethod(httpMethod);

        // Check if method of controller exists, 
        // also if request method is option and controller don't support this method
        // then return response with 200 status
        if (handleFromMethod === undefined) {
            // Return immediately Response With 200 Status code
            if (httpMethod === "OPTIONS")
                return new Response();
            else
                return METHOD_NOT_ALLOWED();
        }

        let response: Response;

        try {
            response = await handleFromMethod();
        } catch (error) {
            response = this.errorToResponse(error);
        }

        // Apply middlewares to response and request also Connection
        for await (const middleware of this.middlewares) {
            middleware(request, response)
        }

        return response;
    }

    /**
     * Coverts Error to response
     * @param error 
     */
    private errorToResponse(error: Error): Response {
        if (error instanceof SyntaxError)
            return new Response("Syntax error", { status: 500 });

        if (error instanceof CustomError)
            return new Response(`${error.name} Error: ${error.message}`, { status: error.status });

        return new Response("Server error", { status: 500 });
    }

    /**
     * Adds the new Controller To Map
     * @param {Controller} controller Brand New Controller
     * @param {string} name
     * @version 1
     */
    public pushController(controller: ExtendedController, name: string): this {
        if (!this.controllers.has(name))
            this.controllers.set(name, controller);

        return this;
    }

    /**
     * Add Middleware
     * @param mid
     */
    public useMiddleware(mid: Middleware): this {
        this.middlewares.push(mid)

        return this;
    }
}

export { App }