<script lang="ts">
	import { makePoly } from '$lib';
	import type { appContext } from './types';
	let { appCtx = $bindable() }: { appCtx: appContext } = $props();

	let hovered = $state(false);
	let pinned = $state(false);

	const closedShape = makePoly(
		[
			{ x: 0, y: 15 },
			{ x: 4, y: 15 },
			{ x: 4, y: 85 },
			{ x: 0, y: 85 }
		],
		2
	);

	const openShape = makePoly(
		[
			{ x: 0, y: 0, r: 1 },
			{ x: 40, y: 20 },
			{ x: 40, y: 80 },
			{ x: 0, y: 100, r: 1 }
		],
		2
	);

	let d = $state(closedShape);

	const minimize = () => {
		if (pinned) {
			return;
		}
		hovered = false;
		setTimeout(() => {
			if (hovered) {
				return;
			}
			d = closedShape;
		}, 0);
	};

	const expand = () => {
		hovered = true;
		d = openShape;
	};

	const addBot = (e?: MouseEvent, id?: number) => {
		//if (e) {
		//	e.stopPropagation();
		//}
		if (!id) {
			id = appCtx.bots.length;
		}
		appCtx.bots.push({
			script: '',
			id
		});
	};

	for (let i = 0; i < 15; i++) {
		addBot(undefined, i);
	}
</script>

<div class="pointer-events-none absolute left-4 z-10 flex h-full flex-col justify-center">
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<div
		onmouseenter={expand}
		onmouseleave={minimize}
		class="aspect-square h-3/4 duration-400 {pinned
			? 'bg-black/75'
			: 'bg-black/50'} pointer-events-auto"
		style="clip-path: {d};"
	>
		<div class="flex h-full w-2/5 items-center justify-center">
			<ul class="custom-scroll flex h-3/5 w-11/12 flex-col gap-1 overflow-y-auto px-1">
				{#each appCtx.bots as bot}
					<button class="rounded bg-black text-white {hovered ? 'block' : 'hidden'}"
						>{bot.id}</button
					>
				{/each}
				<button class="rounded bg-black text-white {hovered ? 'block' : 'hidden'}" onclick={addBot}
					>+</button
				>
			</ul>
		</div>
	</div>
</div>

<style>
	.custom-scroll {
		scrollbar-color: gray black;
		scrollbar-width: thin;
	}
</style>
