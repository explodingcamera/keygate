import { Link, Route, Router, Switch, useLocation, useRouter } from "wouter";
import { Login, Signup } from "@keygate/react";
import { Button, Card, Flex } from "@radix-ui/themes";

export const AuthRoutes = () => {
	const router = useRouter();
	const [location] = useLocation();

	return (
		<Router base="/auth" parent={router}>
			<Flex justify={"end"}>
				<Link href={location.includes("/login") ? "/signup" : "/login"}>
					<Button variant="soft" size={"3"}>
						{location.includes("/login") ? "Sign up" : "Log in"}
					</Button>
				</Link>
			</Flex>
			<Flex grow={"1"} justify={"center"} align={"center"}>
				<Switch>
					<Route path="/login">
						<Login />
					</Route>
					<Route path="/signup" component={Signup} />
				</Switch>
			</Flex>
		</Router>
	);
};
