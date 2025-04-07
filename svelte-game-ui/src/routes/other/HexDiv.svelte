<script lang="ts">
	let { children, stroke = 'gray', border = 0.05, fill = 'black', onclick = null } = $props();

	let strokeWidth = border;

	const angle = Math.PI / 3;
	// Create Points for Hexagon
	let points: string = (() => {
		let points: string[] = [];
		for (let i = 0; i < 6; i++) {
			const x = (1 / 2) * (1 + Math.cos(i * angle));
			const y = (1 / 2) * (1 + Math.sin(i * angle));
			points.push(`${x},${y}`);
		}
		return points.join(' ');
	})();
</script>

<div class="relative flex h-full flex-col items-center justify-center">
	<svg class="h-full overflow-visible" viewBox="0 0 1 1">
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<polygon
			{onclick}
			{points}
			{fill}
			{stroke}
			stroke-width={strokeWidth}
			class:pointer-events-auto={onclick}
		>
		</polygon>
	</svg>
	<div class="absolute text-white">
		{@render children()}
	</div>
</div>
