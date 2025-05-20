<script lang="ts">
	import { wrap, type Remote } from 'comlink';
	import BotMenu from './BotMenu.svelte';
	import GameElem from './Game.svelte';
	import { type appContext } from './types';
	import type { Game } from '$lib/game/game-worker';

	const worker = new Worker(new URL('$lib/game/game-worker.ts', import.meta.url), {
		type: 'module'
	});

	const u: Remote<Game> = wrap(worker);

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
	<GameElem bind:appCtx />
</main>
