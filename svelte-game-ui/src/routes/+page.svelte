<script lang="ts">
	import BotMenu from './BotMenu.svelte';
	import Game from './Game.svelte';
	import { type appContext } from './types';

	let app: appContext = $state({ state: 'game', paused: false, selectedBot: 0, bots: [] });
</script>

<main class="relative h-svh bg-black">
	<BotMenu bind:app />
	<Game bind:app />
</main>
