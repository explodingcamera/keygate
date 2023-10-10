import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import Unfonts from "unplugin-fonts/vite";

export default defineConfig({
	plugins: [
		react(),
		Unfonts({
			google: { families: ["Inter"] },
		}),
	],
});
