import { Controller, CustomError } from "lib";
import { QuranText } from 'models';

class Quran extends Controller {
    // Get Suran from quran
    async get() {
        const obj = this.parseId();

        if (obj.from.sura === null || obj.to.sura === null && obj.to.aya !== null)
            await Promise.reject(new CustomError(400, "quran", "Spicify Sura"));

        const result = await QuranText.select('text', 'sura', 'aya')
            .where("sura", ">=", obj.from.sura)
            .where("sura", "<=", obj.to.sura)
            .offset(parseInt(obj.from.aya!, 10) - 1)
            .get();

        console.log((result as any).concat(0, parseInt(obj.from.aya!, 10)));

        return Response.json(obj);
    }

    parseId() {
        const id = this.router.getId;
        const splitedId = id?.split('-');
        const sura = splitedId![0] || "";
        const aya = splitedId![1] || "";
        const from = this.parseAyaSura(sura);
        const to = this.parseAyaSura(aya);

        return {
            from: {
                sura: from.sura,
                aya: from.aya
            },
            to: {
                sura: to.sura,
                aya: to.aya
            }
        }
    }

    parseAyaSura(num: string) {
        const splitedNum = num.split(':')
        const sura = splitedNum[0] || null;
        const aya = splitedNum[1] || null;

        return {
            sura,
            aya
        }
    }
}

export default Quran;