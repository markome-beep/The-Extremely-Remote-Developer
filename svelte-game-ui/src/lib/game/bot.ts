import type { appContext } from "../../routes/types";
import type { Bot } from "./game-worker";

const updateBots = async (appCtx: appContext, newData: Bot[]) => {
	newData.forEach((b) => {
		let renderedBot = appCtx.bots.find((currB) => {
			currB.id == b.id
		});

		if (renderedBot === undefined) {
			renderedBot = {
			};
			appCtx.bots.push(renderedBot);
		}



	})
}
