import { Controller, SchemaValidator } from 'lib';
import { Model } from "denodb";
import { VerifyCode } from 'models';
import { TokenGenerator, stringToBytes } from './token.ts';

const sendCodeSchema = new SchemaValidator({
    email: { type: "string", required: true, maxLength: 30 }
});

const verifyCodeSchema = new SchemaValidator({
    email: { type: "string", required: true, maxLength: 30 },
    code: { type: "number", required: true, maxLength: 6 }
});

class Account extends Controller {
    post() {
        if (this.router.getAction === "verify") {
            return this.verify();
        } else if (this.router.getAction === "sendCode") {
            return this.sendCode();
        }
        return Promise.resolve(new Response("Action is not Correct"))
    }

    async verify(): Promise<Response> {
        // Get Phone number from Body
        const body = await this.request.json();

        await verifyCodeSchema.validate(body);

        const lastSendedCode = await VerifyCode.select("email", "code", "created_at", "status")
            .where({ email: body.email })
            .orderBy("created_at", "desc")
            .limit(1)
            .first() as Model;

        if (!lastSendedCode)
            return new Response("No code sended to this email!", { status: 403 });

        const currentDate = Date.now();
        const codeCreatedAtDate = Date.parse(lastSendedCode.createdAt as string);

        if (lastSendedCode.status === "used")
            return new Response("This Code is used!");

        if (currentDate - codeCreatedAtDate > 70000)
            return new Response("Code is deprecated!");

        if (lastSendedCode.code === body.code) {
            // Update Status Of Code
            VerifyCode.where("code", lastSendedCode.code as number).update({ status: "used" });

            const newToken = new TokenGenerator(stringToBytes(JSON.stringify(body)), Date.now() * (Math.floor(Math.random() * 0xFFFF)), 100);
            await newToken.generate();

            return Response.json({ token: newToken.getTokenAsString });
        }

        return new Response("Code is not currect!", { status: 400 })
    }

    async sendCode(): Promise<Response> {
        const body = await this.request.json();

        await sendCodeSchema.validate(body);

        const randomCode = await this.generateRandomCode();
        const lastSendedCode = await VerifyCode.select("code", "created_at").where({ email: body.email })
            .orderBy("created_at", "desc")
            .limit(1)
            .first() as Model;

        if (lastSendedCode) {
            const currentDate = Date.now();
            const createdAtAsMilliSecond = Date.parse(lastSendedCode.createdAt! as string);

            if (currentDate - createdAtAsMilliSecond < 5000) {
                return Response.json({ message: "Code is sended." })
            }
        }

        VerifyCode.create({ status: "notUsed", code: randomCode, email: body.email });

        return Response.json({ message: "Code is sended." });
    }

    private generateRandomCode(length = 6): Promise<number> {
        const min = Math.pow(10, (length - 1));
        const max = Math.pow(10, (length));
        const val = Math.floor(Math.random() * (max - min) + min);

        return Promise.resolve(val);
    }
}

export default Account;