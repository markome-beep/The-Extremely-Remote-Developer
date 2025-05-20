import type { Game, Bot } from "$lib/game/game-worker";
import type { Remote } from "comlink";
import type { Container, Sprite } from "pixi.js";

export type applicationState = 'game' | 'editor';
export type appContext = {
	state: applicationState;
	bots: bot[];
	gameData: Remote<Game>;
	renderData: RenderData
	stage?: Container
	//selectedBot:
	//| 0
	//| 1
	//| 2
	//| 3
	//| 4
	//| 5
	//| 6
	//| 7
	//| 8
	//| 9
	//| 10
	//| 11
	//| 12
	//| 13
	//| 14
	//| 15
	//| 16
	//| 17
	//| 18
	//| 19;
};

type tile = {
	x: number,
	y: number,
	sprite: Sprite
}

export type bot = {
	script: string,
	id: number
	state: Bot
	sprite: Sprite
}

type RenderData = {
	tiles: tile[]
	bots: bot[]
}

