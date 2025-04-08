import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

const basePath = process.env.BASE_PATH || ''; // important for github pages

const config = {
	preprocess: vitePreprocess(),
	kit: { adapter: adapter() },
	paths: {
		base: basePath
	}
};

export default config;
