<script lang="ts">
	import { basicSetup } from 'codemirror';
	import { EditorView, keymap } from '@codemirror/view';
	import { Compartment } from '@codemirror/state';
	import { oneDark } from '@codemirror/theme-one-dark';
	import { indentWithTab } from '@codemirror/commands';
	import { vim } from '@replit/codemirror-vim';
	import { onMount } from 'svelte';
	import { parser } from '$lib/editor/lezer';
	import { LanguageSupport, LRLanguage } from '@codemirror/language';

	let div: HTMLDivElement;
	let vim_mode: Boolean = true;
	let editorView: EditorView;
	const vimConf = new Compartment();

	const example = LRLanguage.define({
		parser
	});

	onMount(() => {
		editorView = new EditorView({
			doc: 'Start document',
			extensions: [
				vimConf.of(vim_mode ? vim() : []), // Vim toggle
				basicSetup,
				oneDark,
				keymap.of([indentWithTab]),
				new LanguageSupport(example)
			],
			parent: div,

			dispatch: async (transaction) => {
				console.log(editorView.state.doc.toString());
				editorView.update([transaction]);
			}
		});
	});

	const toggleVim = () => {
		vim_mode = !vim_mode;
		editorView.dispatch({
			effects: vimConf.reconfigure(
				vim_mode ? vim() : [] // Vim toggle
			)
		});
		editorView.focus();
	};
</script>

<div class="flex h-screen flex-col gap-2 bg-blue-400 p-1">
	<button onclick={toggleVim} class="rounded border px-1 shadow" class:bg-gray-700={vim_mode}
		>Toggle</button
	>
	<div
		class="m-2 h-full overflow-clip rounded-xl bg-[#282c34] p-1"
		onclick={() => editorView.focus()}
	>
		<div bind:this={div}></div>
	</div>
</div>
