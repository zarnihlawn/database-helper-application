<script lang="ts">
	import type { QueryBlockInterface } from '$lib/model/interface/schema.interface';
	import { invoke } from '@tauri-apps/api/core';
	import * as monaco from 'monaco-editor';
	import { editor } from 'monaco-editor';

	let {
		block,
		language = 'markdown',
		theme = 'vs',
		readOnly = false,
		onChange = () => {}
	} = $props<{
		block: QueryBlockInterface;
		language?: 'markdown' | 'sql' | 'json';
		theme?: 'vs' | 'vs-dark' | 'hc-black';
		readOnly?: boolean;
		onChange?: (value: string) => void;
	}>();

	let editorInstance: editor.IStandaloneCodeEditor;
	let container: HTMLDivElement;

	function updateHeight() {
		if (!editorInstance) return;

		const content = editorInstance.getValue();
		const lineCount = content.split('\n').length;
		const lineHeight = 19; // Monaco's default line height
		const padding = 40; // Increased padding
		const newHeight = Math.max(lineHeight, lineCount * lineHeight + padding);

		container.style.height = `${newHeight}px`;
		editorInstance.layout();
	}

	$effect(() => {
		editorInstance = monaco.editor.create(container, {
			language,
			theme,
			readOnly,
			automaticLayout: true,
			minimap: { enabled: false },
			scrollBeyondLastLine: false,
			fontSize: 14,
			lineNumbers: 'on',
			roundedSelection: false,
			padding: { top: 10, bottom: 10 },
			scrollbar: {
				vertical: 'visible',
				horizontal: 'visible',
				useShadows: false,
				verticalScrollbarSize: 10,
				horizontalScrollbarSize: 10
			},
			value: block.query_content_block
		});

		// Apply border radius to editor container
		const editorContainer = container.querySelector('.monaco-editor');
		if (editorContainer) {
			(editorContainer as HTMLElement).style.borderRadius = '10px';
		}

		// Initial height setup
		updateHeight();

		// Handle content changes
		editorInstance.onDidChangeModelContent(() => {
			const newValue = editorInstance.getValue();
			onChange(newValue);
			updateHeight();
			saveQueryContentToTheBlock(block.id, newValue);
		});

		return () => {
			if (editorInstance) {
				editorInstance.dispose();
			}
		};
	});

	async function saveQueryContentToTheBlock(
		queryBlockId: number,
		queryContentBlock: string
	) {
		await invoke('save_query_content_to_the_block', {
			queryBlockId: queryBlockId,
			queryContentBlock: queryContentBlock
		});
	}

	$effect(() => {
		if (editorInstance) {
			monaco.editor.setModelLanguage(editorInstance.getModel()!, language);
		}
	});

	$effect(() => {
		if (editorInstance) {
			monaco.editor.setTheme(theme);
		}
	});
</script>

<div class="monaco-editor w-full overflow-hidden" bind:this={container}></div>

<style>
	.monaco-editor {
		border-radius: 10px;
		border: 1px solid var(--border-color, #ccc);
		transition: height 0.2s ease-in-out;
		padding: 10px 0; /* Add padding to container */
	}

	:global(.monaco-editor .overflow-guard) {
		border-radius: 10px;
	}

	:global(.monaco-editor .monaco-scrollable-element) {
		border-radius: 10px;
	}

	:global(.monaco-editor .view-lines) {
		padding: 10px 0; /* Add padding to content */
	}
</style>
