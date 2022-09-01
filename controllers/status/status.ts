import { Controller } from 'lib';

class Status extends Controller {
    get() {
        return Promise.resolve(new Response("Hello World"))
    }
}

export default Status;