<script lang="ts">
	let { children, stroke = 'gray', size = 100, border = 0.1, fill = 'black', ...props } = $props();

	let strokeWidth = (size / 2) * border;

	const angle = Math.PI / 3;
	let points: string = (() => {
		let points: string[] = [];
		for (let i = 0; i < 6; i++) {
			const x = size / 2 + (size / 2) * Math.cos(i * angle);
			const y = size / 2 + (size / 2) * Math.sin(i * angle);
			points.push(`${x},${y}`);
		}
		return points.join(' ');
	})();
</script>

<svg viewBox="0 0 {size} {size}" height={size} width={size} style="overflow: visible;">
	<polygon {points} {fill} {stroke} stroke-width={strokeWidth}> </polygon>
	<foreignObject x="0" y="0" height={size} width={size}>
		<div {...props}>
			{@render children()}
		</div>
	</foreignObject>
</svg>
