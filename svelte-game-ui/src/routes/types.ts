import type { GameData } from "wasm-game-state";

export type applicationState = 'game' | 'editor';
export type appContext = {
	state: applicationState;
	paused: boolean;
	bots: bot[];
	gameData: GameData;
	selectedBot:
	| 0
	| 1
	| 2
	| 3
	| 4
	| 5
	| 6
	| 7
	| 8
	| 9
	| 10
	| 11
	| 12
	| 13
	| 14
	| 15
	| 16
	| 17
	| 18
	| 19;
};
export type bot = {
	script: string,
	id: number
}
