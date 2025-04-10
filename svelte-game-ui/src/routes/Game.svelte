<script lang="ts">
	import { init_game } from '$lib/game/init-game';
	import { Application } from 'pixi.js';
	import { onMount } from 'svelte';
	import type { appContext } from './types';

	let { appCtx: appCtx = $bindable() }: { appCtx: appContext } = $props();

	let game_canvas: HTMLCanvasElement;
	let container: HTMLDivElement;
	let fps = $state(0);

	const app = new Application();

	const game = async () => {
		// Initialize the application
		await app.init({
			background: '#1099bb',
			canvas: game_canvas,
			resizeTo: container
		});
		app.stage.hitArea = app.screen;

		await init_game(app.stage, appCtx.gameData);

		let timeAccum = 0;
		const tickThreshold = 10;
		app.ticker.add((ticker) => {
			timeAccum += ticker.deltaTime;
			fps = (1 / ticker.deltaTime) * app.ticker.FPS;
			if (timeAccum > tickThreshold) {
				timeAccum -= tickThreshold;
			}
		});
	};

	onMount(game);
</script>

<div class="h-full w-full p-2">
	<div class="pointer-events-none absolute top-3 right-4 text-white">FPS: {fps.toFixed(1)}</div>
	<div class="h-full overflow-clip rounded-2xl" bind:this={container}>
		<div class="h-0">
			<canvas class="w-full" bind:this={game_canvas}></canvas>
		</div>
	</div>
</div>
