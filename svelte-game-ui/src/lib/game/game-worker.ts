import * as Comlink from 'comlink';
import init, { GameData } from '$lib/wasm';

const load_wasm = (async () => {
	await init();
	return GameData.paths();
})();

let u = new Proxy(
	{},
	{
		get(_, prop) {
			return async (...args: any) => {
				console.log(`Calling: ${prop}`);
				const realObj = await load_wasm;
				const method: any = realObj[prop];
				if (typeof method !== "function") throw new Error(`'${prop}' is not a function`);
				const val = method.apply(realObj, args);
				console.log(val);

				// If result is an object (likely a WASM class instance), proxy it
				if (typeof val === 'object' && val !== null) {
					return Comlink.proxy(val);
				}

				return val;
			};
		}
	}
);

Comlink.expose(u);
