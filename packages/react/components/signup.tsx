import { Box, Button, Flex, Heading, Text, TextField } from "@radix-ui/themes";
import { Seperator, AuthForm } from "./ui";

export const Signup = () => {
	return (
		<AuthForm>
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
					<Button variant="outline" color="gray" size={"3"}>
						Google
					</Button>
				</fieldset>
			</Flex>
		</AuthForm>
	);
};
