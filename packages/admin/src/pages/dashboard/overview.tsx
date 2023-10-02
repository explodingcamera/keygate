import { Card, Flex, Grid, Heading } from "@radix-ui/themes";

export const Overview = () => {
	return (
		<Flex width={"100%"}>
			<Grid
				width={"100%"}
				gap="3"
				columns={{
					sm: "2",
					md: "4",
				}}
			>
				<Card>
					<Heading size={"2"}>Overview</Heading>
				</Card>{" "}
				<Card>
					<Heading size={"2"}>Overview</Heading>
				</Card>{" "}
				<Card>
					<Heading size={"2"}>Overview</Heading>
				</Card>{" "}
				<Card>
					<Heading size={"2"}>Overview</Heading>
				</Card>
			</Grid>
		</Flex>
	);
};
