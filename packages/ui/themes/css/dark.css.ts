import { createTheme } from "@vanilla-extract/css";
import { vars } from "..";

export const darkTheme = createTheme(vars, {
	color: {
		brand: "red",
	},
});
