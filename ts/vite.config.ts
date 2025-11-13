import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],

	// Tauri expects a fixed port on localhost
	server: {
		port: 5173,
		strictPort: true,
		host: 'localhost'
	},

	// Prevent vite from obscuring rust errors
	clearScreen: false,

	// Tauri uses a different watch mode
	envPrefix: ['VITE_', 'TAURI_'],

	build: {
		// Tauri uses Chromium on Windows and WebKit on macOS and Linux
		target: process.env.TAURI_PLATFORM === 'windows' ? 'chrome105' : 'safari13',
		// Don't minify for debug builds
		minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
		// Produce sourcemaps for debug builds
		sourcemap: !!process.env.TAURI_DEBUG
	}
});
