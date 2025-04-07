<script lang="ts">
	import { Tween } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	import type { appContext } from './types';
	import HexDiv from './other/HexDiv.svelte';
	let { app = $bindable() }: { app: appContext } = $props();

	const ops = {
		duration: 200,
		easing: cubicOut
	};

	let width = new Tween(1 / 20, ops);
	let height = 1;
	let angle = new Tween(Math.PI / 2, ops);
	let radius = new Tween(0.2 / 20);
	let hovered = false;
	let pinned = $state(false);
	let open = $state(false);

	const minimize = () => {
		if (pinned) {
			return;
		}
		hovered = false;
		setTimeout(() => {
			if (hovered) {
				return;
			}
			open = false;
			//height.target = 20;
			width.target = 1 / 20;
			angle.target = Math.PI / 2;
			radius.target = 0.2 / 20;
		}, 10);
	};

	const expand = () => {
		hovered = true;
		width.target = 5 / 20;
		angle.target = Math.PI / 3;
		radius.target = 0.5 / 20;
		setTimeout(() => {
			if (!hovered) {
				return;
			}
			open = true;
		}, 300);
	};

	// This was figured out at 1am. I will not be taking further questions
	let d = $derived(`
	M ${Math.sin(angle.current) * radius.current} ${Math.cos(angle.current) * radius.current}
	L ${width.current - Math.sin(angle.current) * radius.current} ${width.current / Math.tan(angle.current) - Math.cos(angle.current) * radius.current}
	C ${width.current} ${width.current / Math.tan(angle.current)} ${width.current} ${width.current / Math.tan(angle.current)} ${width.current} ${width.current / Math.tan(angle.current) + radius.current}
	L ${width.current} ${height - (width.current / Math.tan(angle.current) + radius.current)}
	C ${width.current} ${height - width.current / Math.tan(angle.current)} ${width.current} ${height - width.current / Math.tan(angle.current)} ${width.current - Math.sin(angle.current) * radius.current} ${height - (width.current / Math.tan(angle.current) - Math.cos(angle.current) * radius.current)}
	L ${Math.sin(angle.current) * radius.current} ${height - Math.cos(angle.current) * radius.current}
	C 0 ${height} 0 ${height} 0 ${height - radius.current}
	L 0 ${radius.current}
	C 0 0 0 0 ${Math.sin(angle.current) * radius.current} ${Math.cos(angle.current) * radius.current}
	`);

	for (let i = 0; i < 15; i++) {
		app.bots.push({
			script: '',
			id: i
		});
	}
</script>

<div class="absolute left-4 h-full">
	<div class="relative flex h-full flex-col justify-center">
		{#if open}
			<!--
-->
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div
				onmouseenter={expand}
				onmouseleave={minimize}
				class="clipped-content pointer-events-none absolute flex h-3/4 w-full flex-col justify-center overflow-x-hidden overflow-y-scroll"
			>
				<ul class="ml-3 flex flex-col py-12">
					{#each app.bots as bot}
						<li class="odd:aspect-square odd:w-20 even:relative even:-top-10 even:left-17 even:h-0">
							<div class="aspect-square w-20">
								<HexDiv>{bot.id}</HexDiv>
							</div>
						</li>
					{/each}
					<!-- <li class="odd:aspect-square odd:w-20 even:relative even:-top-10 even:left-17 even:h-0"> -->
					<!-- 	<div class="aspect-square w-20"> -->
					<!-- 		<HexDiv onclick={addBot}>+</HexDiv> -->
					<!-- 	</div> -->
					<!-- </li> -->
				</ul>
			</div>
		{/if}

		<!-- I have to leave the view box as 1x1 for clip to work	 -->
		<svg viewBox="0 0 1 1" class="h-3/4">
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<!-- svelte-ignore a11y_click_events_have_key_events -->
			<path
				onclick={() => {
					if (pinned) {
						pinned = false;
						minimize();
					} else {
						pinned = true;
						expand();
					}
				}}
				class="cursor-pointer"
				onmouseenter={expand}
				onmouseleave={minimize}
				{d}
				fill-opacity={pinned ? 0.7 : 0.4}
			/>
			<clipPath id="myClip" clipPathUnits="objectBoundingBox">
				<path {d} />
			</clipPath>
		</svg>
	</div>
</div>

<style>
	.clipped-content {
		clip-path: url(#myClip);
	}
</style>
