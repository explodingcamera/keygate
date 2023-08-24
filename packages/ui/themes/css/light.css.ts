import { createTheme } from "@vanilla-extract/css";
import { vars } from "..";

export const lightTheme = createTheme(vars, {
	color: {
		brand: "red",
	},
});
