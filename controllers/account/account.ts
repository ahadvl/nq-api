import { Controller, Schema } from 'lib';

const schema = new Schema({ phoneNumber: "number" });

class Account extends Controller {
    get() {
        console.log(this.request)
        return Promise.resolve(new Response("GET FUCK"));
    }

    // @actions({ "login": this.login, "register": this.register })
    post() {
        if (this.router.getAction === "verify") {
            return this.verify();
        }
        return Promise.resolve(new Response("Action is not Correct"))
    }

    async verify(): Promise<Response> {
        // Get Phone number from Body
        const body = await this.request.json();

        if (schema.check(body) === false) {
            return new Response("Request body is not correct!")
        }

        return new Response("GET FUCK");
    }
}

export default Account;