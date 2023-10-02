import { Flex } from "@radix-ui/themes";

export const AuthForm = ({ children }: { children: React.ReactNode }) => (
	<Flex
		p="2"
		direction={"column"}
		justify={"center"}
		className={"__keygate__form"}
	>
		{children}
	</Flex>
);

export const Seperator = ({ children }: { children: React.ReactNode }) => {
	return (
		<Flex
			align={"center"}
			justify={"center"}
			mb={"2"}
			className={"__keygate__seperator"}
		>
			<span className="__keygate__seperator_bar" />
			{children}
			<span className="__keygate__seperator_bar" />
		</Flex>
	);
};
