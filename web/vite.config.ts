import { defineConfig } from "vite";
import preact from "@preact/preset-vite";
import { resolve } from "node:path";

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [preact()],
	build: {
		rollupOptions: {
			input: {
				main: resolve(__dirname, "_standalone/index.html"),
				admin: resolve(__dirname, "_admin/index.html"),
			},
		},
	},
});
