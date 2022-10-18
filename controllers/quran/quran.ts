import { Model } from "https://deno.land/x/denodb@v1.0.40/mod.ts";
import { Controller, CustomError } from "lib";
import { QuranSurah, QuranText } from "models";

class Quran extends Controller {
    async get() {
        if (this.router.getId === null)
            await Promise.reject(new CustomError(400, "quran", "Spicify Sura"));

        const surah = await QuranSurah.select("id", "name", "period").where("id", this.router.getId).first();

        const verses = await QuranText.select("verse", "text")
            .where("surah", surah.id as number)
            .orderBy("verse")
            .get() as Model[];

        return Response.json({
            id: surah.id,
            name: surah.name,
            period: surah.period,
            verses: verses,
        });
    }
}

export default Quran;
