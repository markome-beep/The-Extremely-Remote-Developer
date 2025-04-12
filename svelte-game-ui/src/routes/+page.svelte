<script lang="ts">
	import { type GameData } from '$lib/wasm';
	import { wrap, type Remote } from 'comlink';
	import BotMenu from './BotMenu.svelte';
	import Game from './Game.svelte';
	import { type appContext } from './types';

	const worker = new Worker(new URL('$lib/game/game-worker.ts', import.meta.url), {
		type: 'module'
	});

	const u: Remote<GameData> = wrap(worker);

	let appCtx: appContext = $state({
		state: 'game',
		paused: false,
		selectedBot: 0,
		bots: [],
		gameData: u
	});
</script>

<main class="relative h-dvh overflow-hidden bg-black">
	<BotMenu bind:appCtx />
	<Game bind:appCtx />
</main>
