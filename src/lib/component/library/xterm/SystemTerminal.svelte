<script lang="ts">
	import { Terminal as XTerm } from 'xterm';
	import { FitAddon } from 'xterm-addon-fit';
	import { WebLinksAddon } from 'xterm-addon-web-links';
	import 'xterm/css/xterm.css';
	import { onMount, onDestroy } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';

	let terminal: XTerm;
	let terminalElement: HTMLElement;
	let fitAddon: FitAddon;
	let currentLine = $state('');
	let commandHistory = $state<string[]>([]);
	let historyIndex = $state(-1);
	let currentDirectory = $state('');

	async function executeCommand(command: string) {
		try {
			const output = await invoke('execute_shell_command', { command });
			if (output) {
				terminal.writeln(output as string);
			}
		} catch (error) {
			terminal.writeln(`Error: ${error}`);
		}
	}

	function handleTerminalInput(data: string) {
		const code = data.charCodeAt(0);

		// Handle backspace
		if (code === 127) {
			if (currentLine.length > 0) {
				currentLine = currentLine.slice(0, -1);
				terminal.write('\b \b');
			}
			return;
		}

		// Handle enter
		if (code === 13) {
			terminal.write('\r\n');

			if (currentLine.trim()) {
				// Add to history
				commandHistory = [...commandHistory, currentLine];
				historyIndex = commandHistory.length;

				// Execute command
				executeCommand(currentLine);

				// Clear current line
				currentLine = '';
			}

			// Write new prompt with current directory
			const prompt = currentDirectory ? `[${currentDirectory}] $ ` : '$ ';
			terminal.write(prompt);
			return;
		}

		// Handle arrow keys
		if (code === 27) {
			const key = data.slice(1);
			if (key === '[A') {
				// Up arrow
				if (historyIndex > 0) {
					historyIndex--;
					currentLine = commandHistory[historyIndex];
					terminal.write('\r\x1b[K$ ' + currentLine);
				}
			} else if (key === '[B') {
				// Down arrow
				if (historyIndex < commandHistory.length - 1) {
					historyIndex++;
					currentLine = commandHistory[historyIndex];
					terminal.write('\r\x1b[K$ ' + currentLine);
				} else {
					historyIndex = commandHistory.length;
					currentLine = '';
					terminal.write('\r\x1b[K$ ');
				}
			}
			return;
		}

		// Handle printable characters
		if (code >= 32 && code <= 126) {
			currentLine += data;
			terminal.write(data);
		}
	}

	onMount(() => {
		// Initialize terminal
		terminal = new XTerm({
			cursorBlink: true,
			fontSize: 14,
			fontFamily: 'Menlo, Monaco, "Courier New", monospace',
			theme: {
				background: '#1a1a1a',
				foreground: '#ffffff',
				cursor: '#ffffff',
				black: '#000000',
				red: '#cd3131',
				green: '#0dbc79',
				yellow: '#e5e510',
				blue: '#2472c8',
				magenta: '#bc3fbc',
				cyan: '#11a8cd',
				white: '#e5e5e5',
				brightBlack: '#666666',
				brightRed: '#f14c4c',
				brightGreen: '#23d18b',
				brightYellow: '#f5f543',
				brightBlue: '#3b8eea',
				brightMagenta: '#d670d6',
				brightCyan: '#29b8db',
				brightWhite: '#e5e5e5'
			}
		});

		// Add addons
		fitAddon = new FitAddon();
		terminal.loadAddon(fitAddon);
		terminal.loadAddon(new WebLinksAddon());

		// Open terminal in the element
		terminal.open(terminalElement);

		// Fit terminal to container
		fitAddon.fit();

		// Handle window resize
		window.addEventListener('resize', () => {
			fitAddon.fit();
		});

		// Write welcome message
		terminal.writeln('Welcome to System Terminal');
		terminal.writeln('Type your commands here...');
		terminal.write('\r\n$ ');

		// Handle input
		terminal.onData(handleTerminalInput);
	});

	onDestroy(() => {
		if (terminal) {
			terminal.dispose();
		}
	});
</script>

<div class="terminal-container h-[400px] w-full p-4">
	<div bind:this={terminalElement} class="h-full w-full"></div>
</div>

<style>
	.terminal-container {
		background-color: #1a1a1a;
		border-radius: 0.5rem;
	}
</style>
