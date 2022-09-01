import { Middleware } from "lib";

/**
 * Cors of response
 * @param _req 
 * @param res 
 * @param origin 
 * @param creden 
 */
const cors: Middleware = (
    _req: Request,
    res: Response,
    origin = "*",
    creden = "true"
): void => {
    res.headers.append("Access-Control-Allow-Origin", origin);
    res.headers.append("Access-Control-Allow-Credentials", creden);
}

export {
    cors
}