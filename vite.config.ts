import { purgeCss } from 'vite-plugin-tailwind-purgecss';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import Icons from 'unplugin-icons/vite';

export default defineConfig({
	plugins: [sveltekit(), purgeCss(), Icons({ compiler: 'svelte' })],
	test: {
		globals: true,
		environment: 'jsdom',
		include: ['src/**/*.{test,spec}.{js,ts}'],
		exclude: ['**/node_modules/**', '**/dist/**', '**/build/**'],
		setupFiles: ['./src/test/setup.ts'],
		coverage: {
			reporter: ['text', 'json', 'html'],
			exclude: ['**/*.d.ts', '**/*.spec.ts', '**/*.test.ts']
		}
	}
});