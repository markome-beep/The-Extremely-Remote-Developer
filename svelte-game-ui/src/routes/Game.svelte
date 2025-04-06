<script lang="ts">
	import { Application } from 'pixi.js';
	import { onMount } from 'svelte';

	let { app = $bindable() } = $props();

	let game_canvas: HTMLCanvasElement;
	let container: HTMLDivElement;
	let fps = $state(0);

	const game = async () => {
		const app = new Application();

		// Initialize the application
		await app.init({
			background: '#1099bb',
			canvas: game_canvas,
			resizeTo: container
		});
		app.stage.hitArea = app.screen;

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
	<div class="absolute flex w-full flex-row justify-center"></div>
	<div class="h-full overflow-clip rounded-2xl" bind:this={container}>
		<div class="image-pixelated h-0">
			<canvas class="w-full" bind:this={game_canvas}></canvas>
		</div>
	</div>

	<style>
		.image-pixelated {
			image-rendering: pixelated;
		}
	</style>
</div>
