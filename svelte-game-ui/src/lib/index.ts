import { ast } from "wasm-game-state";

// place files you want to import through the `$lib` alias in this folder.
export const parser = (src: string) => {
	return ast(src)
}
