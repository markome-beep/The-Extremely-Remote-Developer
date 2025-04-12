import { Container } from "pixi.js";
import { load_assets } from "./assets";
import { make_map } from "./tile-map";
import type { GameData } from "$lib/wasm";
import type { Remote } from "comlink";

const make_camera = (stage: Container) => {
	const camera = new Container;

	const zoom_speed = 0.1;
	let is_dragging = false;
	let last_pos = { x: 0, y: 0 };
	let mouse_down_pos = { x: 0, y: 0 }

	stage.on("pointerdown", (event) => {
		is_dragging = true;
		mouse_down_pos = { x: event.clientX, y: event.clientY };
		last_pos = { x: camera.x, y: camera.y };

	})
		.on("pointermove", (event) => {
			if (!is_dragging) return;
			const dx = event.clientX - mouse_down_pos.x + last_pos.x;
			const dy = event.clientY - mouse_down_pos.y + last_pos.y;
			camera.x = dx;
			camera.y = dy;
		})
		.on("pointerupoutside", () => { is_dragging = false })
		.on("pointerup", () => { is_dragging = false })
		.on("wheel", (event) => {
			event.preventDefault();
			const zoom_factor = (event.deltaY || -1) > 0 ? (1 - zoom_speed) : (1 + zoom_speed);

			let mouse = camera.toLocal({ x: event.clientX - 10, y: event.clientY - 10 });

			camera.x += (mouse.x - camera.pivot.x) * camera.scale.x;
			camera.y += (mouse.y - camera.pivot.y) * camera.scale.y;

			camera.pivot.set(mouse.x, mouse.y);
			camera.scale.x *= zoom_factor;
			camera.scale.y *= zoom_factor;
		})

	stage.eventMode = 'static';

	camera.label = 'camera';
	stage.addChild(camera);
	camera.x = 450;
	camera.y = 50;

	return camera
}

export const init_game = async (stage: Container, gameData: Remote<GameData>) => {
	load_assets();
	const camera = make_camera(stage)
	const hexContainer = await make_map(gameData);
	camera.addChild(hexContainer);
}
