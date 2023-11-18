import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
	plugins: [
        wasmPack("./game"),
        sveltekit()
    ],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	}
});
