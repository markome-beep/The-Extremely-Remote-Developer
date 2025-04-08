import { ast } from "wasm-game-state";
import roundPolygon, { getSegments } from 'round-polygon';

// place files you want to import through the `$lib` alias in this folder.
export const parser = (src: string) => {
	return ast(src)
}

export const makePoly = (points: { x: number, y: number, r?: number }[], r: number) => {
	const roundedPoly = roundPolygon(points, r);


	return 'polygon(' +
		getSegments(roundedPoly, 'AMOUNT', 10)
			.map((val) => `${val.x}% ${val.y}%`)
			.join(', ') +
		')';
}
