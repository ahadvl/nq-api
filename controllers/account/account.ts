import { Controller, SchemaValidator } from 'lib';
import { Model } from "denodb";
import { VerifyCode, TokenModel, User } from 'models';
import { Token } from './token.ts';

const sendCodeSchema = new SchemaValidator({
    email: { type: "string", required: true, maxLength: 30, minLength: 13 },
});

const verifyCodeSchema = new SchemaValidator({
    email: { type: "string", required: true, maxLength: 30, minLength: 13 },
    code: { type: "number", required: true, maxLength: 6, minLength: 6 }
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
            return new Response("Code is not valid", { status: 404 });

        const currentDate = Date.now();
        const codeCreatedAtDate = Date.parse(lastSendedCode.createdAt as string);

        if (lastSendedCode.status === "used")
            return new Response("Code is not valid!");

        if (currentDate - codeCreatedAtDate > 70000)
            return new Response("Code is not valid!");

        if (lastSendedCode.code !== body.code)
            return new Response("Code is not valid!");

        // Update Status Of Code
        VerifyCode.where("code", lastSendedCode.code as number).update({ status: "used" });

        const user = await User.select().where({ email: body.email }).first();
        let userId: number;

        if (!user) {
            const newUser = await User.create({ email: body.email });
            const newUserId = newUser.id as number;
            newUser.username = `u${newUserId}`;
            await newUser.update();

            userId = newUserId;
        } else {
            userId = user.id as number;
        }

        const newToken = new Token(userId);
        await newToken.generate();

        await TokenModel.create({ token: newToken.getTokenAsString, userId: userId })

        return Response.json({ token: newToken.getTokenAsString });
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