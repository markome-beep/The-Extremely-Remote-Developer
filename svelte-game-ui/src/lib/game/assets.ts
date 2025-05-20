import { base } from "$app/paths";
import { Assets } from "pixi.js";

export const load_assets = async () => {
	const manifest = {
		bundles: [
			{
				name: "tiles",
				assets: [
					{
						alias: 'grass',
						src: `${base}/sprites/Hex_Tile.png`
					},
					{
						alias: 'path',
						src: `${base}/sprites/Path_Tile.png`
					},
					{
						alias: 'wall',
						src: `${base}/sprites/Wall_Tile.png`
					}
				]
			},
			{
				name: "bot",
				assets: [
					{
						alias: 'bot',
						src: `${base}/sprites/Up_Bot.png`
					},
				]
			},
		],
	};

	await Assets.init({ manifest });
	Assets.backgroundLoadBundle(['tiles', 'bot']);
}
