import "./styles.css";
import "@radix-ui/themes/styles.css";

import { Theme } from "@radix-ui/themes";
import { useDarkMode } from "./utils/use-dark-mode";
import { ComponentProps } from "react";

export { Login } from "./components/login";
export { Signup } from "./components/signup";

export const useKeygate = () => {};

export const KeygateProvider = ({
	children,
	accentColor,
}: {
	children: React.ReactNode;
	accentColor?: ComponentProps<typeof Theme>["accentColor"];
}) => {
	const { isDarkMode } = useDarkMode();

	return (
		<Theme
			accentColor={accentColor ?? "teal"}
			appearance={isDarkMode ? "dark" : "light"}
		>
			<div className={"__keygate"}>{children}</div>
		</Theme>
	);
};
