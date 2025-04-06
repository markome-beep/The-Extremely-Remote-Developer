<script lang="ts">
	import BotMenu from './BotMenu.svelte';
	import Game from './Game.svelte';
	import HexDiv from './other/HexDiv.svelte';
	import { type appContext } from './types';

	let app: appContext = $state({ state: 'game', paused: false, selectedBot: 0 });
</script>

<main class="relative h-svh bg-black">
	<BotMenu {app} />
	<Game {app} />
</main>
