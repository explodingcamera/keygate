import { useEffect } from "react";
import { useLocalStorage, useMediaQuery, useUpdateEffect } from "usehooks-ts";

const COLOR_SCHEME_QUERY = "(prefers-color-scheme: dark)";

export function useDarkMode(defaultValue?: boolean): {
	isDarkMode: boolean;
	toggle: () => void;
	enable: () => void;
	disable: () => void;
} {
	const isDarkOS = useMediaQuery(COLOR_SCHEME_QUERY);
	const [isDarkMode, setDarkMode] = useLocalStorage<boolean>(
		"usehooks-ts-dark-mode",
		defaultValue ?? isDarkOS ?? false,
	);

	// Update darkMode if os prefers changes
	useUpdateEffect(() => {
		setDarkMode(isDarkOS);
	}, [isDarkOS]);

	useEffect(() => {
		document.body.classList.toggle("dark", isDarkOS);
	}, [isDarkOS]);

	return {
		isDarkMode,
		toggle: () => setDarkMode((prev) => !prev),
		enable: () => setDarkMode(true),
		disable: () => setDarkMode(false),
	};
}
