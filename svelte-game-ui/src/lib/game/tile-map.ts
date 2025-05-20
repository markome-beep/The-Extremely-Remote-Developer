import { Assets, Container, Polygon, Sprite, Texture } from "pixi.js";
import { TileKinds, type GameData } from "$lib/wasm";
import type { Remote } from "comlink";

export const row_off = [0, 16] as const;
export const col_off = [23, 8] as const;

const hitArea = [
	0, 15,
	8, 7,
	24, 7,
	32, 15,
	24, 24,
	8, 24
].map((e) => e - 16);

export const make_map = async (game: Remote<GameData>) => {
	const hexContainer = new Container();

	const grass: Texture = await Assets.load('grass');
	const empty: Texture = await Assets.load('wall');
	const path: Texture = await Assets.load('wall');
	const wall: Texture = await Assets.load('path');
	grass.source.scaleMode = 'nearest';
	empty.source.scaleMode = 'nearest';
	path.source.scaleMode = 'nearest';
	wall.source.scaleMode = 'nearest';

	let nums = 10;
	for (let i = 0; i < nums * 2 - 1; i++) {
		for (let j = Math.max(-i, -nums + 1); j < Math.min(nums, nums * 2 - i - 1); j++) {
			let t;
			try {
				console.log(`(${i}, ${j})`)
				t = await game.get_tile(i, j)
			} catch {
				return hexContainer;
			}

			let tile;

			switch (await t.kind) {

				case TileKinds.Empty:
					tile = Sprite.from(empty);
					break;

				case TileKinds.Grass:
					tile = Sprite.from(grass);
					break;

				case TileKinds.Wall:
					tile = Sprite.from(wall);
					break;

				case TileKinds.Path:
					tile = Sprite.from(path);
					break;

				default:
					console.error(`Unrecognized Tile Kind: ${t.kind}\nTile Found at (${i}, ${j})`);
					return hexContainer;
			}

			tile.hitArea = new Polygon(hitArea);
			tile.on('pointerover', () => { tile.alpha = 0.5 });
			tile.on('pointerleave', () => { tile.alpha = 1 });
			tile.eventMode = 'static';
			tile.anchor.set(0.5);

			tile.y = (row_off[1] * i + col_off[1] * j);
			tile.x = (row_off[0] * i + col_off[0] * j);

			////Debug hit area
			//const g = new Graphics().poly(hitArea).fill();
			//g.x = tile.x
			//g.y = tile.y
			//hexContainer.addChild(g);

			hexContainer.addChild(tile);
		}
	}

	return hexContainer
}

export const renderTile = async (t: TileKinds, x: number, y: number, map: Container) => {

	let texture: Texture;

	switch (t) {

		case TileKinds.Empty:
			texture = await Assets.load('empty');
			break;

		case TileKinds.Grass:
			texture = await Assets.load('grass');
			break;

		case TileKinds.Wall:
			texture = await Assets.load('wall');
			break;

		case TileKinds.Path:
			texture = await Assets.load('path');
			break;

		default:
			texture = await Assets.load('empty');
			console.error(`Unrecognized Tile Kind: ${t}\nTile Found at (${x}, ${y})`);
	}

	texture.source.scaleMode = 'nearest';
	const tile = Sprite.from(texture);

	tile.hitArea = new Polygon(hitArea);
	tile.on('pointerover', () => { tile.alpha = 0.5 });
	tile.on('pointerleave', () => { tile.alpha = 1 });
	tile.eventMode = 'static';
	tile.anchor.set(0.5);

	tile.y = (row_off[1] * x + col_off[1] * y);
	tile.x = (row_off[0] * x + col_off[0] * y);

	map.addChild(tile);
}
