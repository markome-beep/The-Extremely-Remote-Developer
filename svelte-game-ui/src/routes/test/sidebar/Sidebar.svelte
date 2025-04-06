<script>
	let isHovered = false;
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	class="fixed top-0 left-0 flex h-screen transition-all duration-300 {isHovered
		? 'w-48'
		: 'w-12'} bg-gray-800 text-white shadow-md"
	on:mouseenter={() => (isHovered = true)}
	on:mouseleave={() => (isHovered = false)}
>
	{#if isHovered}
		<div class="flex w-full flex-col items-center justify-center">
			<div class="hex-container">
				<div class="hex-row">
					<span class="hexagon">Home</span>
					<span class="hexagon">About</span>
				</div>
				<div class="hex-row">
					<span class="hexagon">Services</span>
					<span class="hexagon">Contact</span>
				</div>
			</div>
		</div>
	{:else}
		<div class="flex w-full flex-col items-center justify-center">
			<svg
				class="h-6 w-6"
				fill="none"
				stroke="currentColor"
				viewBox="0 0 24 24"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M4 6h16M4 12h16M4 18h16"
				/>
			</svg>
		</div>
	{/if}
</div>

<style>
	.hex-container {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.hex-row {
		display: flex;
	}

	.hexagon {
		width: 60px;
		height: 34.64px; /*  height = width * sqrt(3) / 2 */
		background-color: #3498db;
		color: white;
		display: flex;
		justify-content: center;
		align-items: center;
		margin: 5px;
		clip-path: polygon(25% 0%, 75% 0%, 100% 50%, 75% 100%, 25% 100%, 0% 50%);
		transition: background-color 0.3s ease;
	}

	.hexagon:hover {
		background-color: #2980b9;
	}
</style>
