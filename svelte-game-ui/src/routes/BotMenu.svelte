<script lang="ts">
	import { Tween } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	import type { appContext } from './types';
	let { app = $bindable() }: { app: appContext } = $props();

	const ops = {
		duration: 200,
		easing: cubicOut
	};

	let width = new Tween(0.5, ops);
	let height = new Tween(20);
	let angle = new Tween(Math.PI / 2, ops);
	let radius = new Tween(0.2);
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
			width.target = 0.5;
			angle.target = Math.PI / 2;
			radius.target = 0.2;
		}, 10);
	};

	const expand = () => {
		hovered = true;
		//height.target = 20;
		width.target = 5;
		angle.target = Math.PI / 3;
		radius.target = 0.5;
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
	L ${width.current} ${height.current - (width.current / Math.tan(angle.current) + radius.current)}
	C ${width.current} ${height.current - width.current / Math.tan(angle.current)} ${width.current} ${height.current - width.current / Math.tan(angle.current)} ${width.current - Math.sin(angle.current) * radius.current} ${height.current - (width.current / Math.tan(angle.current) - Math.cos(angle.current) * radius.current)}
	L ${Math.sin(angle.current) * radius.current} ${height.current - Math.cos(angle.current) * radius.current}
	C 0 ${height.current} 0 ${height.current} 0 ${height.current - radius.current}
	L 0 ${radius.current}
	C 0 0 0 0 ${Math.sin(angle.current) * radius.current} ${Math.cos(angle.current) * radius.current}
	`);
</script>

<div class="absolute left-4 flex h-full flex-col justify-center">
	{#if open}
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<!--
		<div onmouseenter={expand} onmouseleave={minimize} class="absolute h-1/2 bg-blue-400">
			<ul>
				<li>
					<button class="cursor-pointer"> TEST </button>
				</li>
			</ul>
		</div>
-->
	{/if}

	<svg viewBox="0 0 {1} {20}" class="h-3/4 overflow-visible">
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
	</svg>
</div>
