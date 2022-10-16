import { Model } from "https://deno.land/x/denodb@v1.0.40/mod.ts";
import { Controller, CustomError } from "lib";
import { QuranSura, QuranText } from "models";

class Quran extends Controller {
    async get() {
        if (this.router.getId === null)
            await Promise.reject(new CustomError(400, "quran", "Spicify Sura"));

        const sura = await QuranSura.select("id", "name", "period").where("id", this.router.getId).first();

        const suraAyas = await QuranText.select('aya', 'text')
            .where("sura", sura.id as number)
            .orderBy("aya")
            .get() as Model[];

        return Response.json({
            id: sura.id,
            name: sura.name,
            period: sura.period,
            ayas: suraAyas,
        });
    }
}

export default Quran;
