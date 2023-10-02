import "./styles.css";
import "@radix-ui/themes/styles.css";

import {
	Box,
	Button,
	Flex,
	Heading,
	Text,
	TextField,
	Theme,
} from "@radix-ui/themes";
import { useDarkMode } from "./utils/use-dark-mode";
import { ComponentProps } from "react";

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

export const Form = () => {};

export const Login = () => {
	return (
		<Flex direction={"column"} justify={"center"} className={"__keygate__form"}>
			<Heading align={"center"} mb="2">
				Create an account
			</Heading>
			<Text size={"2"} align={"center"} mb="5">
				Enter your email address below to create an account
			</Text>
			<Flex display={"flex"} direction={"column"} asChild>
				<fieldset>
					<TextField.Input placeholder="name@example.com" size={"3"} mb={"2"} />
					<Button variant="solid" size={"3"} mb="6">
						Continue
					</Button>
					<Seperator>
						<Text
							color="gray"
							style={{ textTransform: "uppercase" }}
							size={"1"}
						>
							or continue with
						</Text>
					</Seperator>
					<Box mb="5" />
					<Button variant="outline" color="gray" size={"3"} mb="2">
						Google
					</Button>
					<Button variant="outline" color="gray" size={"3"} mb="2">
						Google
					</Button>
				</fieldset>
			</Flex>
		</Flex>
	);
};

const Seperator = ({ children }: { children: React.ReactNode }) => {
	return (
		<Flex
			align={"center"}
			justify={"center"}
			mb={"2"}
			className={"__keygate__seperator"}
		>
			{children}
		</Flex>
	);
};

export const Signup = () => {
	return (
		<Box>
			<Heading>Create an account</Heading>
			<fieldset>
				<TextField.Input />
			</fieldset>
		</Box>
	);
};
