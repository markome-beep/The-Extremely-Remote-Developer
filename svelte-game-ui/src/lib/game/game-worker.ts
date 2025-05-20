import * as Comlink from 'comlink';
import init, { BotLite, Dir, GameData, TileKinds, type BotState } from '$lib/wasm';

export type Bot = {
	x: number,
	y: number,
	dir: Dir
	state: BotState
	id: number
}

export type Game = {
	initGame: () => Promise<boolean>;
	gameData?: GameData
	tick: () => Bot[]
	getTile: (x: number, y: number) => TileKinds
}

const u: Game = {
	async initGame() {
		try {
			await init();
			this.gameData = GameData.paths();
			return true
		}
		catch (e) {
			return false
		}
	},

	tick() {
		let x: BotLite[] = this.gameData!.tick();

		return x.map((b) => {
			return {
				x: b.x,
				y: b.y,
				dir: b.dir,
				state: b.state,
				id: b.id
			}
		});
	},

	getTile(x: number, y: number) {
		let t = this.gameData!.get_tile(x, y);
		return t.kind;
	}
}

Comlink.expose(u);
