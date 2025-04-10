import { base } from "$app/paths";
import { Assets, Spritesheet, type SpritesheetData, AnimatedSprite, Container, Sprite } from "pixi.js";

export const load_assets = async () => {
	const manifest = {
		bundles: [
			{
				name: "tiles",
				assets: [
					{
						alias: 'grass',
						src: `${base}/sprites/Hex_Tile.png`
					}
				]
			},
		],
	};

	await Assets.init({ manifest });
	Assets.backgroundLoadBundle(['tiles']);
}
