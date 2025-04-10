<script lang="ts">
	import { GameData } from 'wasm-game-state';
	import BotMenu from './BotMenu.svelte';
	import Game from './Game.svelte';
	import { type appContext } from './types';

	let appCtx: appContext = $state({
		state: 'game',
		paused: false,
		selectedBot: 0,
		bots: [],
		gameData: GameData.new()
	});
</script>

<main class="relative h-dvh overflow-hidden bg-black">
	<BotMenu bind:appCtx />
	<Game bind:appCtx />
</main>
